use std::collections::LinkedList;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchResponse {
    pub result: LinkedList<String>
}

#[derive(Serialize, Deserialize)]
pub struct InfoResponse {
    pub name: String,
    pub state: InfoStateKind,
    pub start_time: String,
    pub end_time: String,
    pub holidays: LinkedList<String>
}

#[derive(Serialize, Deserialize)]
pub enum InfoStateKind {
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "BEFORE_OPEN")]
    BeforeOpen,
    #[serde(rename = "AFTER_CLOSED")]
    AfterClosed,
    #[serde(rename = "HOLIDAY_CLOSED")]
    HolidayClosed
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
