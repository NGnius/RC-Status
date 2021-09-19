use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CardlifeContext {
    pub debug: String,
    pub indicators: crate::status::ServiceData,
    pub server_list: Vec<super::server_list::ServerItem>,
}

impl CardlifeContext {
    pub fn new() -> Self {
        Self {
            debug: "Hello World!".to_string(),
            indicators: crate::status::ServiceData::new(),
            server_list: Vec::new(),
        }
    }
}
