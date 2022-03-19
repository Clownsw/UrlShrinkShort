use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Msg {
    pub code: u32,
    pub message: String,
}

impl Msg {
    pub fn new() -> Self {
        Msg {
            code: 500,
            message: "".to_string(),
        }
    }
}
