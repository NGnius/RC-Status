// This should probably use a proper database, but JSON is good enough for now
use serde::{Deserialize, Serialize};
use std::thread::{spawn, sleep, JoinHandle};
use chrono::prelude::{Utc, DateTime};
use chrono::serde::ts_seconds;
use chrono::TimeZone;

crate::lazy_static! {
    //static ref DATA: RwLock<PersistentData> = RwLock::new(PersistentData::new());
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PersistentData {
    incidents: Vec<Incident>,
    stats: Vec<DataPoint>,
}

impl PersistentData {
    pub fn new() -> PersistentData {
        PersistentData {
            incidents: vec![],
            stats: vec![]
        }
    }

    pub fn push_incident(&mut self, i: Incident, max: usize) {
        if self.incidents.len() == max {
            self.incidents.remove(0);
        }
        self.incidents.push(i);
    }

    pub fn push_stat(&mut self, d: DataPoint, max: usize) {
        if self.stats.len() == max {
            self.stats.remove(0);
        }
        self.stats.push(d);
    }

    pub fn datapoints(&self) -> &[DataPoint] {
        &self.stats
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Incident {
    Custom {
        #[serde(with = "ts_seconds")]
        time: DateTime<Utc>,
        #[serde(with = "ts_seconds")]
        resolved: DateTime<Utc>,
        title: String,
        description: String,
    },
    Maintenance {
        #[serde(with = "ts_seconds")]
        time: DateTime<Utc>,
        #[serde(with = "ts_seconds")]
        resolved: DateTime<Utc>,
        message: String,
    },
    MiscOutage {
        #[serde(with = "ts_seconds")]
        time: DateTime<Utc>,
        #[serde(with = "ts_seconds")]
        resolved: DateTime<Utc>,
        title: String,
        description: String,
    }
}

impl Incident {
    pub fn resolve(&mut self, endtime: DateTime<Utc>) {
        if self.is_resolved() { return;}
         match self {
             Incident::Custom { resolved, .. } => *resolved = endtime,
             Incident::Maintenance { resolved, .. } => *resolved = endtime,
             Incident::MiscOutage { resolved, .. } => *resolved = endtime,
         }
    }

    pub fn is_resolved(&self) -> bool {
        self.resolved() != &epoch()
    }

    pub fn resolved(&self) -> &DateTime<Utc> {
        match self {
            Incident::Custom { resolved, .. } => resolved,
            Incident::Maintenance { resolved, .. } => resolved,
            Incident::MiscOutage { resolved, .. } => resolved,
        }
    }

    pub fn variation(&self) -> usize {
        match self {
            Incident::Custom { .. } => 1,
            Incident::Maintenance { .. } => 2,
            Incident::MiscOutage { .. } => 4,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DataPoint{
    #[serde(with = "ts_seconds")]
    pub time: DateTime<Utc>,
    pub max: f32,
    pub min: f32,
    pub avg: f32,
}

pub fn start_worker() -> JoinHandle<()> {
    if let Ok(json_file) = std::fs::File::open("data.json") {
        // populate persistent data, if exists
        let data = serde_json::from_reader(std::io::BufReader::new(json_file))
            .expect("Failed to parse data.json");
        crate::CONTEXT.write().unwrap().data = data;
    }
    {
        crate::CONTEXT.write().unwrap().data_ok = true;
    }
    spawn(persistence_worker)
}

fn persistence_worker() {
    while ! *crate::IS_STOPPING.read().unwrap() {
        let writer = std::io::BufWriter::new(
            std::fs::File::create("data.json").expect("Failed to create data.json")
        );
        { // DATA read scope
            serde_json::to_writer(writer, &crate::CONTEXT.write().unwrap().data)
                .expect("Failed to save data.json");
        }
        // no API spam
        sleep(std::time::Duration::from_millis(crate::CONFIG.read().unwrap().period_ms));
    }
}

pub fn report(incident: Incident) {
    crate::CONTEXT.write().unwrap().data.push_incident(incident, crate::CONFIG.read().unwrap().max_incidents);
}

pub fn resolve(variant: usize, endtime: DateTime<Utc>) {
    for incident in &mut crate::CONTEXT.write().unwrap().data.incidents {
        if incident.variation() == variant {
            incident.resolve(endtime);
        }
    }
}

pub fn collect(data: DataPoint) {
    crate::CONTEXT.write().unwrap().data.push_stat(data, crate::CONFIG.read().unwrap().max_datapoints);
}

pub fn time_now() -> DateTime<Utc> {
    Utc::now()
}

pub fn epoch() -> DateTime<Utc> {
    Utc.ymd(1970, 1, 1).and_hms(0, 1, 1)
}
