use std::collections::LinkedList;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchResponse {
    pub result: LinkedList<String>
}

pub struct InfoResponse {
    pub name: String,
    pub state: String,
    pub start_time: String,
    pub end_time: String,
    pub holidays: LinkedList<String>
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
