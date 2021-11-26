use std::thread::{spawn, sleep, JoinHandle};
use crate::persist;

#[allow(dead_code)]
static DOMAIN: &str = "http://robocraftstaticdata.s3.amazonaws.com";

pub fn start_worker() -> JoinHandle<()> {
    spawn(maintenance_worker)
}

fn maintenance_worker() {
    //return;
    let mut old_sd: Option<crate::staticdata::StaticData> = None;
    while ! *crate::IS_STOPPING.read().unwrap() {
        // check if maintenance is starting/stopping
        let mut maintenace_msg: Option<persist::Incident> = None;
        let mut maintenance_resolved = false;
        { // CONTEXT read lock scope (to not lock during sleep)
            let ctx = crate::CONTEXT.read().unwrap();
            if let Some(some_old_sd) = old_sd.clone() {
                //let some_old_sd = old_sd.clone().unwrap();
                if ctx.staticdata_ok {
                    // check for maintenance
                    if some_old_sd.MaintenanceMode != ctx.staticdata.MaintenanceMode {
                        // maintenance mode has changed since last time
                        if ctx.staticdata.MaintenanceMode {
                            // entering maintenance mode
                            maintenace_msg = Some(persist::Incident::Maintenance {
                                time: persist::time_now(),
                                resolved: persist::epoch(),
                                message: ctx.staticdata.MaintenanceMessage.clone(),
                            });
                        } else {
                            // exiting maintenance mode
                            maintenance_resolved = true;
                        }
                    }
                } else {
                    // report error
                    // this leaks important info so it's disabled
                    /*maintenace_msg = Some(persist::Incident::MiscOutage {
                        time: persist::time_now(),
                        resolved: persist::epoch(),
                        title: "Static Data Server Error".to_string(),
                        description: format!("This may prevent you from logging in. Error details: Error in response from {}", DOMAIN),
                    });*/
                }
            }
            if ctx.staticdata_ok {
                old_sd = Some(ctx.staticdata.clone());
            }
        }
        if let Some(msg) = maintenace_msg {
            persist::report(msg);
        } else if maintenance_resolved {
            persist::resolve(2, persist::time_now());
        }
        // no API spam
        sleep(std::time::Duration::from_millis(crate::CONFIG.read().unwrap().period_ms));
    }
}
