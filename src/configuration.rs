// This should probably use a proper database, but JSON is good enough for now
use serde::{Deserialize, Serialize};
use std::thread::{spawn, sleep, JoinHandle};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub max_incidents: usize,
    pub max_datapoints: usize,
    pub graph_ratio: usize,
    pub password: String,
    pub period_ms: u64,
    pub max_graphpoints: usize,
    pub cardlife_email: String,
    pub cardlife_password: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub valid: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            max_incidents: 0,
            max_datapoints: 0,
            graph_ratio: 0,
            password: "".to_string(),
            period_ms: 0,
            max_graphpoints: 0,
            cardlife_email: "".to_string(),
            cardlife_password: "".to_string(),
            valid: false,
        }
    }
}

pub fn start_worker() -> JoinHandle<()> {
    // initial load
    {// CONFIG write scope
        let json_file = std::fs::File::open("config.json").expect("Failed to open config.json");
        let config = serde_json::from_reader(std::io::BufReader::new(json_file)).expect("Failed to parse config.json");
        *crate::CONFIG.write().unwrap() = config;
    }
    spawn(configloader_worker)
}

fn configloader_worker() {
    while ! *crate::IS_STOPPING.read().unwrap() {
        {// CONFIG write scope
            let json_file = std::fs::File::open("config.json").expect("Failed to open config.json");
            let config = serde_json::from_reader(std::io::BufReader::new(json_file)).expect("Failed to parse config.json");
            *crate::CONFIG.write().unwrap() = config;
        }
        // no API spam
        sleep(std::time::Duration::from_millis(crate::CONFIG.read().unwrap().period_ms));
    }
}
