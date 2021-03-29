use std::thread::{spawn, sleep, JoinHandle};

use crate::ping::ping;

const INDICATOR_NAME: &str = "CDN";

pub fn start_worker() -> JoinHandle<()> {
    spawn(payment_worker)
}

fn payment_worker() { // lol
    while ! *crate::IS_STOPPING.read().unwrap() {
        let staticdata_ok = crate::CONTEXT.read().unwrap().staticdata_ok;
        if staticdata_ok {
            // to prevent long read lock, clone first then strip off port number
            let full_addr = crate::CONTEXT.read().unwrap().staticdata.AvatarCdnUrl.clone();
            let addr = full_addr.split("/").collect::<Vec<&str>>()[0];
            if let Ok(ping_time) = ping(addr) {
                crate::CONTEXT.write().unwrap().indicators.update(INDICATOR_NAME, true, ping_time.avg);
            } else {
                crate::CONTEXT.write().unwrap().indicators.update_error(INDICATOR_NAME, true);
            }
        }
        // no API spam
        let dur = crate::CONFIG.read().unwrap().period_ms;
        sleep(std::time::Duration::from_millis(dur));
    }
}