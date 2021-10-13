use std::collections::LinkedList;

use crate::{request::request_emart, response_type::SearchResponse};
use regex::Regex;
use request::{request_costco, request_homeplus};
use response_type::ErrorResponse;
use serde_json::Value;
use urlencoding::decode;
use worker::*;

mod request;
mod response_type;
mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
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
                        "emart" => {
                            let response_body = request_emart(2021, 10, keyword).await?;

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

                            console_log!("{}", response_body);

                            let regex = Regex::new(r"<a href='(.*)'>([가-힣]+점)</a>").unwrap();

                            let mut result: LinkedList<String> = LinkedList::new();

                            for cap in regex.captures_iter(&response_body) {
                                result.push_back(format!("{:?}", cap));
                            }                            
                            
                            // for element in document.select(&Selector::parse("li.clearfix span.name a").unwrap()) {
                            //     result.push_back(element.text().collect::<String>());
                            // }
                            
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
        .get_async("/info/:mart/:keyword", |_, ctx| async move {
            if let Some(mart) = ctx.param("mart") {
                if let Some(keyword) = ctx.param("keyword") {
                    match mart.as_str() {
                        "emart" => {
                            let response_body = request_emart(2021, 10, keyword).await?;

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
                            // let response_body = request_homeplus(keyword).await?;

                            let result: LinkedList<String> = LinkedList::new();
                            
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
        .run(req, env)
        .await
}
