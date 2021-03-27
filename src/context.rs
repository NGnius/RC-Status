use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct IndexContext {
    pub debug: String,
    pub staticdata: crate::staticdata::StaticData,
    pub staticdata_ok: bool,
}

impl IndexContext {
    pub fn new() -> IndexContext
    {
        IndexContext{
            debug: "Hello World!".to_string(),
            staticdata: crate::staticdata::StaticData::new(),
            staticdata_ok: false,
        }
    }
}