use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct IndexContext {
    pub debug: String,
    pub staticdata: crate::staticdata::StaticData,
    pub staticdata_ok: bool,
    pub data: crate::persist::PersistentData,
    pub data_ok: bool,
    pub graph: crate::graphing::GraphData,
    pub game_status: crate::status::ServiceStatus,
    pub indicators: crate::status::ServiceData,
}

impl IndexContext {
    pub fn new() -> IndexContext
    {
        IndexContext{
            debug: "Hello World!".to_string(),
            staticdata: crate::staticdata::StaticData::new(),
            staticdata_ok: false,
            data: crate::persist::PersistentData::new(),
            data_ok: false,
            graph: crate::graphing::GraphData::new(),
            game_status: crate::status::ServiceStatus::new("Game Services".to_string()),
            indicators: crate::status::ServiceData::new(),
        }
    }
}