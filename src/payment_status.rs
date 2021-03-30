use std::thread::{spawn, sleep, JoinHandle};
use reqwest::blocking::ClientBuilder;
use chrono::prelude::{Utc};

const INDICATOR_NAME: &str = "Payment";

pub fn start_worker() -> JoinHandle<()> {
    spawn(payment_worker)
}

fn payment_worker() {
    let mut sleep_dur = std::time::Duration::from_millis(crate::CONFIG.read().unwrap().period_ms);
    let http_client = ClientBuilder::new()
        .connect_timeout(sleep_dur)
        .timeout(sleep_dur)
        .build().expect("Failed to build payment worker HTTP client");
    while ! *crate::IS_STOPPING.read().unwrap() {
        let staticdata_ok = crate::CONTEXT.read().unwrap().staticdata_ok;
        if staticdata_ok {
            // to prevent long read lock, clone first then strip off port number
            let full_addr = crate::CONTEXT.read().unwrap().staticdata.paymentUrl.clone();
            //let addr = full_addr.split("/").collect::<Vec<&str>>()[2];
            let req = http_client.get(&full_addr);
            let start = Utc::now();
            let result = req.send();
            let duration = ((Utc::now() - start).num_microseconds().unwrap() as f32)/1000.0;
            if let Ok(resp) = result {
                if resp.status() == 404 || resp.status() == 200 {
                    crate::CONTEXT.write().unwrap().indicators.update(INDICATOR_NAME, true, duration, true);
                } else {
                    crate::CONTEXT.write().unwrap().indicators.update_error(INDICATOR_NAME, true, true);
                }
            } else {
                crate::CONTEXT.write().unwrap().indicators.update_error(INDICATOR_NAME, true, true);
            }
        }
        // no API spam
        let dur = crate::CONFIG.read().unwrap().period_ms;
        sleep_dur = std::time::Duration::from_millis(dur);
        sleep(sleep_dur);
    }
}
