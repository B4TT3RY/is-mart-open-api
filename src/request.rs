use urlencoding::encode;
use worker::{Error, Fetch, Headers, Method, Request, RequestInit, wasm_bindgen::JsValue};

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Safari/605.1.15";

const EMART_BASE_URL: &str = "https://store.emart.com/branch/searchList.do";
const HOMEPLUS_BASE_URL: &str = "https://corporate.homeplus.co.kr/STORE/HyperMarket.aspx";
const COSTCO_BASE_URL: &str = "https://www.costco.co.kr/store-finder/search?q=";

const HOMEPLUS_VIEWSTATE: &str = "/wEPDwUJLTc2MDkzMDI3D2QWAmYPZBYCAgUPZBYCAgEPZBYCAgEPEGRkFgFmZBgBBR5fX0NvbnRyb2xzUmVxdWlyZVBvc3RCYWNrS2V5X18WAwUkY3RsMDAkQ29udGVudFBsYWNlSG9sZGVyMSRzdG9yZXR5cGUxBSRjdGwwMCRDb250ZW50UGxhY2VIb2xkZXIxJHN0b3JldHlwZTIFJGN0bDAwJENvbnRlbnRQbGFjZUhvbGRlcjEkc3RvcmV0eXBlM+aYO9PJofU5uQQJJZRZ2bboir3I";

pub async fn request_emart(year: i32, month: u32, mart_type: &str, keyword: &str) -> Result<String, Error> {
    let mut headers = Headers::new();
    headers.set("User-Agent", USER_AGENT)?;
    headers.set("Content-Type", "application/x-www-form-urlencoded")?;
    let mut request = RequestInit::new();
    request
        .with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(JsValue::from_str(&format!(
            "srchMode=jijum&year={}&month={}&jMode=true&strConfirmYN=N&searchType={}&keyword={}",
            year,
            month,
            mart_type,
            keyword
        ))));

    let mut response = Fetch::Request(Request::new_with_init(
        EMART_BASE_URL,
        &request,
    )?)
    .send()
    .await?;

    response.text().await
}

pub async fn request_homeplus(keyword: &str) -> Result<String, Error> {
    let mut headers = Headers::new();
    headers.set("User-Agent", USER_AGENT)?;
    headers.set("Content-Type", "application/x-www-form-urlencoded")?;
    let mut request = RequestInit::new();
    request
        .with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(JsValue::from_str(&format!(
            "__VIEWSTATE={}&ctl00$ContentPlaceHolder1$srch_name={}&ctl00$ContentPlaceHolder1$storetype1=on",
            encode(HOMEPLUS_VIEWSTATE).into_owned(),
            keyword
        ))));

    let mut response = Fetch::Request(Request::new_with_init(
        HOMEPLUS_BASE_URL,
        &request,
    )?)
    .send()
    .await?;

    response.text().await
}

pub async fn request_costco(keyword: &str) -> Result<String, Error> {
    let mut headers = Headers::new();
    headers.set("User-Agent", USER_AGENT)?;
    let mut request = RequestInit::new();
    request
        .with_method(Method::Get)
        .with_headers(headers);

    let mut response = Fetch::Request(Request::new_with_init(
        format!("{}{}", COSTCO_BASE_URL, keyword).as_str(),
        &request,
    )?)
    .send()
    .await?;

    response.text().await
}