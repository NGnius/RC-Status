use serde::{Deserialize, Serialize};

const PING_SLOW_THRESHOLD: f32 = 200.0;
const PING_OFFLINE_THRESHOLD: f32 = crate::graphing::GRAPH_MAXIMUM_VALUE * 0.9999;
//const ROUND_MAGNITUDE: f32 = 100.0;

#[derive(Serialize, Deserialize, Clone)]
pub struct ServiceStatus {
    pub name: String,
    pub ping: f32,
    pub ping_i: usize,
    pub color: String,
    pub bg_color: String,
    pub text: String,
    pub ok: bool,
    pub ignore_slow: bool,
}

impl ServiceStatus {
    pub fn new(name: String) -> ServiceStatus {
        ServiceStatus{
            name,
            ping: 0.0,
            ping_i: 0,
            color: "".to_string(),
            bg_color: "".to_string(),
            text: "".to_string(),
            ok: false,
            ignore_slow: false,
        }
    }

    pub fn update(&mut self, set_text: bool, ping: f32) {
        self.ping = ping;
        if ping < crate::graphing::GRAPH_MINIMUM_VALUE
            || ping > crate::graphing::GRAPH_MAXIMUM_VALUE {
            self.ping = crate::graphing::GRAPH_MAXIMUM_VALUE;
        }
        self.ping_i = self.ping.round() as usize;
        self.ok = true;
        if self.ping > PING_OFFLINE_THRESHOLD {
            if set_text {
                self.text = "Offline".to_string();
            }
            self.color = "#ff1111".to_string();
            self.bg_color = "#aa1111".to_string();
        } else if self.ping > PING_SLOW_THRESHOLD && !self.ignore_slow {
            if set_text {
                self.text = "Slow".to_string();
            }
            self.color = "#ffff11".to_string();
            self.bg_color = "#aaaa11".to_string();
        } else {
            if set_text {
                self.text = "Online".to_string();
            }
            self.color = "#11ff11".to_string();
            self.bg_color = "#11aa11".to_string();
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ServiceData {
    services: Vec<ServiceStatus>,
}

impl ServiceData {
    pub fn new() -> ServiceData {
        ServiceData{
            services: vec![],
        }
    }

    pub fn update(&mut self, name: &str, set_text: bool, ping: f32, ignore_slow: bool) {
        for service in &mut self.services {
            if service.name == name {
                service.ignore_slow = ignore_slow;
                service.update(set_text, ping);
                return;
            }
        }
        // create if not found, for startup
        let mut service_status = ServiceStatus::new(name.to_string());
        service_status.ignore_slow = ignore_slow;
        service_status.update(set_text, ping);
        self.services.push(service_status)
    }

    pub fn update_error(&mut self, name: &str, set_text:bool, ignore_slow:bool) {
        self.update(name, set_text, crate::graphing::GRAPH_MAXIMUM_VALUE, ignore_slow);
    }
}
