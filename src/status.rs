use serde::{Deserialize, Serialize};

const PING_SLOW_THRESHOLD: f32 = 200.0;
const PING_OFFLINE_THRESHOLD: f32 = 500.0;

#[derive(Serialize, Deserialize, Clone)]
pub struct ServiceStatus {
    pub name: String,
    pub ping: f32,
    pub color: String,
    pub bg_color: String,
    pub text: String,
    pub ok: bool,
}

impl ServiceStatus {
    pub fn new(name: String) -> ServiceStatus {
        ServiceStatus{
            name,
            ping: 0.0,
            color: "".to_string(),
            bg_color: "".to_string(),
            text: "".to_string(),
            ok: false,
        }
    }

    pub fn update(&mut self, set_text: bool, ping: f32) {
        self.ping = ping;
        if ping < crate::graphing::GRAPH_MINIMUM_VALUE
            || ping > crate::graphing::GRAPH_MAXIMUM_VALUE {
            self.ping = crate::graphing::GRAPH_MAXIMUM_VALUE;
        }
        self.ok = true;
        if self.ping > PING_OFFLINE_THRESHOLD {
            if set_text {
                self.text = "Offline".to_string();
            }
            self.color = "#ff1111".to_string();
            self.bg_color = "#aa1111".to_string();
        } else if self.ping > PING_SLOW_THRESHOLD {
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

    pub fn update(&mut self, name: &str, set_text: bool, ping: f32) {
        for service in &mut self.services {
            if service.name == name {
                service.update(set_text, ping);
                return;
            }
        }
        // create if not found, for startup
        let mut service_status = ServiceStatus::new(name.to_string());
        service_status.update(set_text, ping);
        self.services.push(service_status)
    }

    pub fn update_error(&mut self, name: &str, set_text:bool) {
        self.update(name, set_text, crate::graphing::GRAPH_MAXIMUM_VALUE);
    }
}