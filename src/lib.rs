use std::collections::LinkedList;

use crate::{request::request_emart, response_type::SearchResponse};
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveTime, TimeZone, Timelike, Utc, Weekday};
use chrono_tz::{Asia::Seoul, Tz};
use nipper::Document;
use regex::Regex;
use request::{request_costco, request_homeplus};
use response_type::{ErrorResponse, InfoResponse, InfoStateKind};
use serde_json::Value;
use urlencoding::decode;
use worker::*;

mod request;
mod response_type;
mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Utc::now().with_timezone(&Seoul),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get_async("/search/:mart/:keyword", |_, ctx| async move {
            if let Some(mart) = ctx.param("mart") {
                if let Some(keyword) = ctx.param("keyword") {
                    match mart.as_str() {
                        "emart" | "traders" => {
                            let now = Utc::now().with_timezone(&Seoul);

                            let search_type = match mart.as_str() {
                                "emart" => "EM",
                                "traders" => "TR",
                                _ => ""
                            };

                            let response_body = request_emart(now.year(), now.month(), search_type, keyword).await?;

                            let json: Value = serde_json::from_str(&response_body).unwrap_or_default();
                            let mut result: LinkedList<String> = LinkedList::new();

                            for data in json["dataList"].as_array().unwrap() {
                                result.push_back(data["NAME"].as_str().unwrap().to_string());
                            }

                            if result.is_empty() {
                                return Response::from_json(&ErrorResponse {
                                    error: "검색 결과가 없습니다.".to_string(),
                                });
                            }

                            return Response::from_json(&SearchResponse {
                                result
                            });
                        }
                        "homeplus" => {
                            let response_body = request_homeplus(keyword).await?;

                            let document = Document::from(&response_body);

                            let mut result: LinkedList<String> = LinkedList::new();

                            document.select("ul.result_list > li.clearfix").iter().for_each(|element| {
                                result.push_back(element.select("span.name > a").text().to_string());
                            });                       
                            
                            if result.is_empty() {
                                return Response::from_json(&ErrorResponse {
                                    error: "검색 결과가 없습니다.".to_string(),
                                });
                            }

                            return Response::from_json(&SearchResponse {
                                result
                            });
                        }
                        "costco" => {
                            let response_body = request_costco(keyword).await?;

                            let json: Value = serde_json::from_str(&response_body).unwrap_or_default();
                            let mut result: LinkedList<String> = LinkedList::new();

                            for data in json["data"].as_array().unwrap() {
                                let display_name = data["displayName"].as_str().unwrap().to_string();
                                if !display_name.contains(&decode(keyword).unwrap().into_owned()) {
                                    continue;
                                }
                                result.push_back(data["displayName"].as_str().unwrap().to_string());
                            }

                            if result.is_empty() {
                                return Response::from_json(&ErrorResponse {
                                    error: "검색 결과가 없습니다.".to_string(),
                                });
                            }

                            return Response::from_json(&SearchResponse {
                                result
                            });
                        }
                        _ => {
                            return Response::from_json(&ErrorResponse { error: "지원하지 않는 마트 종류입니다.".to_string() });
                        }
                    }
                }
            }
            Response::error("Bad Request", 400)
        })
        .get_async("/info/:mart/:name", |_, ctx| async move {
            if let Some(mart) = ctx.param("mart") {
                if let Some(name) = ctx.param("name") {
                    match mart.as_str() {
                        "emart" | "traders" => {
                            let now = Utc::now().with_timezone(&Seoul);

                            let search_type = match mart.as_str() {
                                "emart" => "EM",
                                "traders" => "TR",
                                _ => ""
                            };

                            let response_body = request_emart(now.year(), now.month(), search_type, name).await?;

                            let json: Value = serde_json::from_str(&response_body).unwrap_or_default();
                            let json = &json["dataList"][0];

                            if json["NAME"].as_str().unwrap() != decode(name).unwrap().into_owned() {
                                return Response::from_json(&ErrorResponse {
                                    error: "검색 결과가 없습니다.".to_string(),
                                });
                            }

                            let mut holidays: LinkedList<String> = LinkedList::new();

                            if let Some(holiday1) = json["HOLIDAY_DAY1_YYYYMMDD"].as_str() {
                                if !holiday1.is_empty() {
                                    let datetime = DateTime::parse_from_str(&format!("{} 235959 +09:00", holiday1), "%Y%m%d %H%M%S %z").unwrap();
                                    if now <= datetime {
                                        holidays.push_back(holiday1.to_string());
                                    }
                                }
                            }

                            if let Some(holiday2) = json["HOLIDAY_DAY2_YYYYMMDD"].as_str() {
                                if !holiday2.is_empty() {
                                    let datetime = DateTime::parse_from_str(&format!("{} 235959 +09:00", holiday2), "%Y%m%d %H%M%S %z").unwrap();
                                    if now <= datetime {
                                        holidays.push_back(holiday2.to_string());
                                    }
                                }
                            }

                            if let Some(holiday3) = json["HOLIDAY_DAY3_YYYYMMDD"].as_str() {
                                if !holiday3.is_empty() {
                                    let datetime = DateTime::parse_from_str(&format!("{} 235959 +09:00", holiday3), "%Y%m%d %H%M%S %z").unwrap();
                                    if now <= datetime {
                                        holidays.push_back(holiday3.to_string());
                                    }
                                }
                            }

                            let start_time = json["OPEN_SHOPPING_TIME"].as_str().unwrap().to_string();
                            let end_time = json["CLOSE_SHOPPING_TIME"].as_str().unwrap().to_string();

                            let jijum_status = json["JIJUM_STATUS"].as_str().unwrap();

                            let state: InfoStateKind = if jijum_status == "CLOSED" {
                                if holidays.contains(&now.format("%Y%m%d").to_string()) {
                                    InfoStateKind::HolidayClosed
                                } else {
                                    let start = Seoul.ymd(now.year(), now.month(), now.day()).and_hms(start_time[0..2].parse().unwrap(), start_time[3..5].parse().unwrap(), 0);
                                    if now < start {
                                        InfoStateKind::BeforeOpen
                                    } else {
                                        InfoStateKind::AfterClosed
                                    }
                                }
                            } else {
                                InfoStateKind::Open
                            };

                            return Response::from_json(&InfoResponse {
                                name: json["NAME"].as_str().unwrap().to_string(),
                                state,
                                start_time,
                                end_time,
                                holidays
                            });
                        }
                        "homeplus" => {
                            let now = Utc::now().with_timezone(&Seoul);

                            let response_body = request_homeplus(name).await?;

                            let document = Document::from(&response_body);

                            let mut result: Option<InfoResponse> = None;

                            let mart = decode(name).unwrap().into_owned();

                            document.select("ul.result_list > li.clearfix").iter().for_each(|element| {
                                if element.select("span.name > a").text().to_string() == mart {
                                    let time = element.select(".time > span:nth-child(1)").text().to_string();
                                    let time: Vec<&str> = time.split("~").collect();

                                    let start_time: Vec<u32> = time[0].split(":").map(|str| str.parse().unwrap()).collect();
                                    let end_time: Vec<u32> = time[1].split(":").map(|str| str.parse().unwrap()).collect();
                                    let start_time = Seoul.ymd(now.year(), now.month(), now.day()).and_hms(start_time[0], start_time[1], 0);
                                    
                                    let end_time = if end_time[0] <= 23 {
                                        Seoul.ymd(now.year(), now.month(), now.day()).and_hms(end_time[0], end_time[1], 0)
                                    } else {
                                        let now = now.clone() + Duration::days(1);
                                        now.with_hour(end_time[0] - 24).unwrap();
                                        now.with_minute(end_time[1]).unwrap();
                                        now
                                    };

                                    let mut holidays = LinkedList::new();
                                    let holiday = element.select(".time > span.off").text().to_string().replace("-", "");
                                    holidays.push_back(holiday);

                                    let state = if holidays.contains(&now.format("%Y%m%d").to_string()) {
                                        InfoStateKind::HolidayClosed
                                    } else if now < start_time {
                                        InfoStateKind::BeforeOpen
                                    } else if now > end_time {
                                        InfoStateKind::AfterClosed  
                                    } else {
                                        InfoStateKind::Open
                                    };

                                    result = Some(InfoResponse {
                                        name: element.select("span.name > a").text().to_string(),
                                        state,
                                        start_time: time[0].to_string(),
                                        end_time: time[1].to_string(),
                                        holidays,
                                    });

                                    return;
                                }
                            });

                            if let Some(info_response) = result {
                                return Response::from_json(&info_response);
                            }

                            return Response::from_json(&ErrorResponse {
                                error: "검색 결과가 없습니다.".to_string(),
                            });
                        }
                        "costco" => {
                            let now = Utc::now().with_timezone(&Seoul);
                            let response_body = request_costco(name).await?;

                            let json: Value = serde_json::from_str(&response_body).unwrap_or_default();
                            let json = &json["data"][0];

                            let display_name = json["displayName"].as_str().unwrap().to_string();
                            if !display_name.contains(&decode(name).unwrap().into_owned()) {
                                return Response::from_json(&ErrorResponse {
                                    error: "검색 결과가 없습니다.".to_string(),
                                });
                            }

                            let days = vec!["월", "화", "수", "목", "금", "토", "일"];

                            let time = &json["openings"][days[now.weekday() as usize]]["individual"];
                            let time = time.as_str().unwrap().to_string();
                            let time = time
                                .replace("오전", "AM")
                                .replace("오후", "PM");
                            let time: Vec<&str> = time.split(" - ").collect();

                            let start_time = NaiveTime::parse_from_str(time[0], "%p %I:%M").unwrap();
                            let start_time = Seoul.ymd(now.year(), now.month(), now.day()).and_hms(start_time.hour(), start_time.minute(), start_time.second());
                            
                            let end_time = NaiveTime::parse_from_str(time[1], "%p %I:%M").unwrap();
                            let end_time = Seoul.ymd(now.year(), now.month(), now.day()).and_hms(end_time.hour(), end_time.minute(), end_time.second());

                            let holidays = parse_costco_holiday(json["storeContent"].as_str().unwrap().to_string(), &now);

                            let state = if holidays.contains(&now.format("%Y%m%d").to_string()) {
                                InfoStateKind::HolidayClosed
                            } else if now < start_time {
                                InfoStateKind::BeforeOpen
                            } else if now > end_time {
                                InfoStateKind::AfterClosed
                            } else {
                                InfoStateKind::Open
                            };

                            return Response::from_json(&InfoResponse {
                                name: display_name,
                                state,
                                start_time: start_time.format("%H:%M").to_string(),
                                end_time: end_time.format("%H:%M").to_string(),
                                holidays,
                            });
                        }
                        _ => {
                            return Response::from_json(&ErrorResponse { error: "지원하지 않는 마트 종류입니다.".to_string() });
                        }
                    }
                }
            }
            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}

fn parse_costco_holiday(html: String, now: &DateTime<Tz>) -> LinkedList<String> {
    let mut result: LinkedList<String> = LinkedList::new();
    let regex = Regex::new("매월 ([첫둘셋넷])째, ([첫둘셋넷])째 ([월화수목금토일])요일").unwrap();
    if let Some(caps) = regex.captures(&html) {
        let first = match caps.get(1).map_or("", |m| m.as_str()) {
            "첫" => 1,
            "둘" => 2,
            "셋" => 3,
            "넷" => 4,
            _ => 0
        };
        let second = match caps.get(2).map_or("", |m| m.as_str()) {
            "첫" => 1,
            "둘" => 2,
            "셋" => 3,
            "넷" => 4,
            _ => 0
        };
        let weekday = match caps.get(3).map_or("", |m| m.as_str()) {
            "월" => Weekday::Mon,
            "화" => Weekday::Tue,
            "수" => Weekday::Wed,
            "목" => Weekday::Thu,
            "금" => Weekday::Fri,
            "토" => Weekday::Sat,
            "일" => Weekday::Sun,
            _ => Weekday::Mon
        };

        let first_holiday = NaiveDate::from_weekday_of_month(now.year(), now.month(), weekday, first);
        let first_holiday = Seoul.ymd(first_holiday.year(), first_holiday.month(), first_holiday.day()).and_hms(23, 59, 59);
        let second_holiday = NaiveDate::from_weekday_of_month(now.year(), now.month(), weekday, second);
        let second_holiday = Seoul.ymd(second_holiday.year(), second_holiday.month(), second_holiday.day()).and_hms(23, 59, 59);

        if now <= &first_holiday {
            result.push_back(first_holiday.format("%Y%m%d").to_string());
        }

        if now <= &second_holiday {
            result.push_back(second_holiday.format("%Y%m%d").to_string());
        }

        return result;
    }
    
    let regex = Regex::new("매월 ([첫둘셋넷])째 ([월화수목금토일])요일, ([첫둘셋넷])째 ([월화수목금토일])요일").unwrap();
    if let Some(caps) = regex.captures(&html) {
        let first = match caps.get(1).map_or("", |m| m.as_str()) {
            "첫" => 1,
            "둘" => 2,
            "셋" => 3,
            "넷" => 4,
            _ => 0
        };
        let second = match caps.get(3).map_or("", |m| m.as_str()) {
            "첫" => 1,
            "둘" => 2,
            "셋" => 3,
            "넷" => 4,
            _ => 0
        };
        let first_weekday = match caps.get(2).map_or("", |m| m.as_str()) {
            "월" => Weekday::Mon,
            "화" => Weekday::Tue,
            "수" => Weekday::Wed,
            "목" => Weekday::Thu,
            "금" => Weekday::Fri,
            "토" => Weekday::Sat,
            "일" => Weekday::Sun,
            _ => Weekday::Mon
        };
        let second_weekday = match caps.get(4).map_or("", |m| m.as_str()) {
            "월" => Weekday::Mon,
            "화" => Weekday::Tue,
            "수" => Weekday::Wed,
            "목" => Weekday::Thu,
            "금" => Weekday::Fri,
            "토" => Weekday::Sat,
            "일" => Weekday::Sun,
            _ => Weekday::Mon
        };

        let first_holiday = NaiveDate::from_weekday_of_month(now.year(), now.month(), first_weekday, first);
        let first_holiday = Seoul.ymd(first_holiday.year(), first_holiday.month(), first_holiday.day()).and_hms(23, 59, 59);
        let second_holiday = NaiveDate::from_weekday_of_month(now.year(), now.month(), second_weekday, second);
        let second_holiday = Seoul.ymd(second_holiday.year(), second_holiday.month(), second_holiday.day()).and_hms(23, 59, 59);

        if now <= &first_holiday {
            result.push_back(first_holiday.format("%Y%m%d").to_string());
        }

        if now <= &second_holiday {
            result.push_back(second_holiday.format("%Y%m%d").to_string());
        }
        
        return result;
    }

    result
}
