use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertUrl {
    pub url_name: String,
    pub url_target: String,
    pub url_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectUrl {
    pub url_id: i64,
    pub url_name: String,
    pub url_target: String,
    pub url_time: DateTime<Utc>,
}
