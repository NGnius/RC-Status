use std::thread::{spawn, sleep, JoinHandle};
use std::process::Command;
use regex::{RegexBuilder};

use crate::persist;

const PING_TIMEOUT_SECONDS: usize = 20;

pub fn start_worker() -> JoinHandle<()> {
    spawn(ping_worker)
}

fn ping_worker() {
    while ! *crate::IS_STOPPING.read().unwrap() {
        // run ping command to server
        let staticdata_ok= crate::CONTEXT.read().unwrap().staticdata_ok.clone();
        if staticdata_ok {
            let full_addr = crate::CONTEXT.read().unwrap().staticdata.GameplayServerServiceAddress
                .clone();
            let addr = full_addr.split(":").collect::<Vec<&str>>()[0];
            // run ping command to get server ping time
            let ping_result = ping(addr);
            if let Ok(output) = ping_result {
                crate::persist::collect(persist::DataPoint{
                    time: persist::time_now(),
                    min: output.min,
                    avg: output.avg,
                    max: output.max,
                });
                crate::CONTEXT.write().unwrap().game_status.update(true, output.avg);
            } else {
                crate::persist::collect(persist::DataPoint{
                    time: persist::time_now(),
                    min: -1.0,
                    avg: -1.0,
                    max: -1.0,
                });
                crate::CONTEXT.write().unwrap().game_status.update(true, -1.0);
            }
        }
        // no API spam
        let dur = crate::CONFIG.read().unwrap().period_ms;
        sleep(std::time::Duration::from_millis(dur));
    }
}

pub struct PingData {
    pub min: f32,
    pub max: f32,
    pub avg: f32,
}

pub fn ping(addr: &str) -> Result<PingData, ()> {
    let re = RegexBuilder::new(r"([\d\.]+)/([\d\.]+)/([\d\.]+)/([\d\.]+)")
        .build().unwrap();
    let cmd_result = Command::new("ping")
        .args(&["-q", "-c", "4", "-w", &PING_TIMEOUT_SECONDS.to_string(), &addr])
        .output();
    if let Ok(output) = cmd_result {
        let stdout = std::str::from_utf8(&output.stdout).unwrap();
        if let Some(caps) = re.captures(stdout) {
            return Ok(PingData{
                avg:caps.get(2).unwrap().as_str().parse::<f32>().unwrap(),
                min: caps.get(1).unwrap().as_str().parse::<f32>().unwrap(),
                max: caps.get(3).unwrap().as_str().parse::<f32>().unwrap(),
            });
        }
    }
    Err(())
}