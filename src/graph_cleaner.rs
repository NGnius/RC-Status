use std::thread::{spawn, sleep, JoinHandle};

pub fn start_worker() -> JoinHandle<()> {
    //populate_graph_points();
    spawn(cleaner_worker)
}

fn cleaner_worker() {
    //let mut graph_points;
    while ! (*crate::IS_STOPPING.read().unwrap()).clone() {
        let staticdata_ok = crate::CONTEXT.read().unwrap().staticdata_ok.clone();
        let data_ok = crate::CONTEXT.read().unwrap().data_ok.clone();
        if staticdata_ok && data_ok {
            let graph_len = crate::CONTEXT.read().unwrap().graph.datapoints.len();
            let graph_points = crate::CONFIG.read().unwrap().max_graphpoints;
            if graph_len > graph_points {
                // remove every second graph point when too many exist
                let mut ctx = crate::CONTEXT.write().unwrap();
                let mut removed = 0;
                for i in 0..graph_len {
                    if (i & 1) == 0 {
                        let corrected_index = i - removed;
                        println!("Removing point {} (of {})", i, graph_len);
                        let removed_point = ctx.graph.datapoints.remove(corrected_index);
                        if (corrected_index) < ctx.graph.datapoints.len() {
                            ctx.graph.datapoints[corrected_index].merge(removed_point);
                        } else if (corrected_index - 1) < ctx.graph.datapoints.len() {
                            ctx.graph.datapoints[corrected_index - 1].merge(removed_point);
                        }
                        removed += 1;
                    }
                }
            }
        }
        // no CPU spam
        let dur = crate::CONFIG.read().unwrap().period_ms;
        sleep(std::time::Duration::from_millis(dur));
    }
}
