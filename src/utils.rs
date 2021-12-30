use serde::{Deserialize, Serialize};

pub fn current_date() -> String {
    let now = chrono::offset::Utc::now();
    now.to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerError {
    pub data: String,
    pub error: String
}