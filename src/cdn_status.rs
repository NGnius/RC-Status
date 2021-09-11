use std::thread::{spawn, sleep, JoinHandle};
use reqwest::blocking::ClientBuilder;
use chrono::prelude::{Utc};

use crate::ping::ping;

const INDICATOR_NAME: &str = "CDN";

pub fn start_worker() -> JoinHandle<()> {
    spawn(cdn_worker)
}

fn cdn_worker() { // lol
    let mut sleep_dur = std::time::Duration::from_millis(crate::CONFIG.read().unwrap().period_ms);
    let http_client = ClientBuilder::new()
        .connect_timeout(sleep_dur)
        .timeout(sleep_dur)
        .build().expect("Failed to build auth worker HTTP client");
    while ! *crate::IS_STOPPING.read().unwrap() {
        let staticdata_ok = crate::CONTEXT.read().unwrap().staticdata_ok;
        if staticdata_ok {
            // to prevent long read lock, clone first then strip off port number
            let full_addr = crate::CONTEXT.read().unwrap().staticdata.AvatarCdnUrl.clone();
            let addr = full_addr.split("/").collect::<Vec<&str>>()[0];
            if let Ok(ping_time) = ping(addr) {
                let http_addr = "http://".to_owned() + addr;
                //println!("CDN HTTP URL {}", &http_addr);
                let req = http_client.get(&http_addr);
                let start = Utc::now();
                let result = req.send();
                let duration = ((Utc::now() - start).num_microseconds().unwrap() as f32)/1000.0;
                if let Ok(resp) = result {
                    if resp.status() == 404 || resp.status() == 200 {
                        let avg_time = (duration + ping_time.avg) / 2.0;
                        crate::CONTEXT.write().unwrap().indicators.update(INDICATOR_NAME, true, avg_time, true);
                    } else {
                        crate::CONTEXT.write().unwrap().indicators.update_error(INDICATOR_NAME, true, true);
                    }
                } else {
                    crate::CONTEXT.write().unwrap().indicators.update_error(INDICATOR_NAME, true, true);
                }
            } else {
                crate::CONTEXT.write().unwrap().indicators.update_error(INDICATOR_NAME, true, false);
            }
        }
        // no API spam
        let dur = crate::CONFIG.read().unwrap().period_ms;
        sleep_dur = std::time::Duration::from_millis(dur);
        sleep(sleep_dur);
    }
}
