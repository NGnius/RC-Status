use std::thread::{spawn, sleep, JoinHandle};
use std::process::Command;
use regex::{RegexBuilder};

use crate::persist;

const PING_TIMEOUT_SECONDS: usize = 20;

pub fn start_worker() -> JoinHandle<()> {
    spawn(ping_worker)
}

fn ping_worker() {
    let mut old_high_ping = false;
    let mut new_high_ping = false;
    let mut last_avg_latency = 0.0;
    while ! *crate::IS_STOPPING.read().unwrap() {
        println!("Running game status worker");
        // run ping command to server
        let staticdata_ok = crate::CONTEXT.read().unwrap().staticdata_ok.clone();
        if staticdata_ok {
            let is_in_maintenance = crate::CONTEXT.read().unwrap().staticdata.MaintenanceMode;
            let full_addr = crate::CONTEXT.read().unwrap().staticdata.GameplayServerServiceAddress
                .clone();
            let addr = full_addr.split(":").collect::<Vec<&str>>()[0];
            // run ping command to get server ping time
            let ping_result = ping(addr);
            if let Ok(output) = ping_result {
                last_avg_latency = output.avg;
                let datapoint = if output.avg > crate::graphing::GRAPH_MAXIMUM_VALUE
                    || output.avg < crate::graphing::GRAPH_MINIMUM_VALUE {
                    // make all ping times share the same invalid time
                    persist::DataPoint {
                        time: persist::time_now(),
                        min: output.avg,
                        avg: output.avg,
                        max: output.avg,
                    }
                } else {
                    persist::DataPoint {
                        time: persist::time_now(),
                        min: output.min,
                        avg: output.avg,
                        max: output.max,
                    }
                };
                crate::persist::collect(datapoint);
                crate::CONTEXT.write().unwrap().game_status.update(true, output.avg);
                if output.avg > crate::graphing::GRAPH_MAXIMUM_VALUE
                    || output.avg < crate::graphing::GRAPH_MINIMUM_VALUE
                    || output.max > crate::graphing::GRAPH_MAXIMUM_VALUE
                    || output.max < crate::graphing::GRAPH_MINIMUM_VALUE
                    || output.min > crate::graphing::GRAPH_MAXIMUM_VALUE
                    || output.min < crate::graphing::GRAPH_MINIMUM_VALUE {
                    new_high_ping = true;
                } else {
                    new_high_ping = false;
                }

                println!("Updated ping time successfully");
            } else {
                crate::persist::collect(persist::DataPoint{
                    time: persist::time_now(),
                    min: -1.0,
                    avg: -1.0,
                    max: -1.0,
                });
                crate::CONTEXT.write().unwrap().game_status.update(true, -1.0);
                println!("Updated ping time unsuccessfully (bad ping result)");
            }
            if is_in_maintenance {
                let mut lock = crate::CONTEXT.write().unwrap();
                lock.game_status.text = "Maintenance".to_owned();
                lock.game_status.bg_color = "#aaaa11".to_owned();
                lock.game_status.color = "#ffff11".to_owned();
                println!("Entered maintenance mode");
            }
        }
        if new_high_ping && !old_high_ping {
            // ping has just become high
            persist::report(persist::Incident::HighLatency {
                time: persist::time_now(),
                resolved: persist::epoch(),
                title: "High Latency".to_owned(),
                description: format!("Primary game server is very slow or unreachable. Last ping took {}ms.", last_avg_latency),
            })
        }

        if old_high_ping && !new_high_ping {
            // ping has just returned to normal
            persist::resolve(8, persist::time_now());
        }
        old_high_ping = new_high_ping;
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
