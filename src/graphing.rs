use serde::{Deserialize, Serialize};
use std::thread::{spawn, sleep, JoinHandle};
use chrono::prelude::{Utc, DateTime};
use chrono::serde::ts_seconds;

pub const GRAPH_MINIMUM_VALUE: f32 = 0.0; // ms
pub const GRAPH_MAXIMUM_VALUE: f32 = 1000.0; // ms

#[derive(Serialize, Deserialize, Clone)]
pub struct GraphData {
    pub datapoints: Vec<GraphDataPoint>,
}

impl GraphData {
    pub fn new() -> GraphData {
        GraphData{
            datapoints: vec![],
        }
    }

    fn push_stat(&mut self, d: GraphDataPoint, max: usize) {
        if self.datapoints.len() == max {
            self.datapoints.remove(0);
        }
        self.datapoints.push(d);
    }

    fn update_timestamps(&mut self) {
        if self.datapoints.len() == 0 {return;}
        let start = self.datapoints[0].ref_time.timestamp();
        for dp in &mut self.datapoints {
            dp.time = (dp.ref_time.timestamp() - start) as f32;
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GraphDataPoint {
    #[serde(with = "ts_seconds")]
    ref_time: DateTime<Utc>,
    time: f32,
    max: f32,
    min: f32,
    avg: f32,
}

impl GraphDataPoint {
    pub fn from_datapoint(dp: &crate::persist::DataPoint) -> GraphDataPoint {
        let mut max = dp.max;
        if max > GRAPH_MAXIMUM_VALUE { max = GRAPH_MAXIMUM_VALUE; }
        if max < GRAPH_MINIMUM_VALUE { max = GRAPH_MAXIMUM_VALUE;}
        let mut min = dp.min;
        if min > GRAPH_MAXIMUM_VALUE { min = GRAPH_MAXIMUM_VALUE; }
        if min < GRAPH_MINIMUM_VALUE { min = GRAPH_MAXIMUM_VALUE;}
        let mut avg = dp.avg;
        if avg > GRAPH_MAXIMUM_VALUE { avg = GRAPH_MAXIMUM_VALUE; }
        if avg < GRAPH_MINIMUM_VALUE { avg = GRAPH_MAXIMUM_VALUE;}
        GraphDataPoint {
            ref_time: dp.time,
            time: 0.0,
            max,
            min,
            avg
        }
    }
    
    pub fn from_datapoints(datapoints: &Vec<crate::persist::DataPoint>) -> GraphDataPoint {
        let mut max = GRAPH_MINIMUM_VALUE - 1.0;
        let mut min = GRAPH_MAXIMUM_VALUE + 1.0;
        let mut avg = 0.0;
        for dp in datapoints {
            if dp.max > max && !(dp.max < GRAPH_MINIMUM_VALUE) {
                println!("New max {}", dp.max);
                max = dp.max;
            }
            if dp.min < min && !(dp.min < GRAPH_MINIMUM_VALUE) {
                println!("New min {}", dp.min);
                min = dp.min;
            }
            if avg < GRAPH_MAXIMUM_VALUE {
                if dp.avg > GRAPH_MAXIMUM_VALUE {
                    avg = GRAPH_MAXIMUM_VALUE;
                } else if dp.avg < GRAPH_MINIMUM_VALUE {
                    avg = GRAPH_MAXIMUM_VALUE;
                } else {
                    avg += dp.avg / (datapoints.len() as f32);
                }
            }
        }
        if max > GRAPH_MAXIMUM_VALUE { max = GRAPH_MAXIMUM_VALUE; }
        if max < GRAPH_MINIMUM_VALUE { max = GRAPH_MAXIMUM_VALUE; }
        if min > GRAPH_MAXIMUM_VALUE { min = GRAPH_MAXIMUM_VALUE; }
        if min < GRAPH_MINIMUM_VALUE { min = GRAPH_MAXIMUM_VALUE; }
        GraphDataPoint {
            ref_time: datapoints[datapoints.len()-1].time,
            time: 0.0,
            max,
            min,
            avg,
        }
    }
}

pub fn start_worker() -> JoinHandle<()> {
    populate_graph_points();
    spawn(graph_worker)
}

fn graph_worker() {
    let mut graph_ratio = crate::CONFIG.read().unwrap().graph_ratio;
    let mut point_buffer = std::vec::Vec::<crate::persist::DataPoint>::with_capacity(graph_ratio);
    let mut last_time = crate::persist::time_now();
    while ! *crate::IS_STOPPING.read().unwrap() {
        {
            let ctx = crate::CONTEXT.read().unwrap();
            if ctx.data.datapoints().len() != 0 {
                let dp = &ctx.data.datapoints()[ctx.data.datapoints().len()-1];
                if dp.time > last_time {
                    point_buffer.push(dp.clone());
                    last_time = dp.time.clone();
                }
            }
            graph_ratio = crate::CONFIG.read().unwrap().graph_ratio;
        }
        if point_buffer.len() >= graph_ratio {
            let mut ctx = crate::CONTEXT.write().unwrap();
            ctx.graph.push_stat(GraphDataPoint::from_datapoints(&point_buffer), crate::CONFIG.read().unwrap().max_datapoints);
            ctx.graph.update_timestamps();
            point_buffer.clear();
        }
        // no CPU spam
        sleep(std::time::Duration::from_millis(crate::CONFIG.read().unwrap().period_ms / 2));
    }
}

fn populate_graph_points() {
    let mut ctx = crate::CONTEXT.write().unwrap();
    let graph_ratio = crate::CONFIG.read().unwrap().graph_ratio;
    let mut point_buffer = std::vec::Vec::<crate::persist::DataPoint>::with_capacity(graph_ratio);
    let max_datapoints = crate::CONFIG.read().unwrap().max_datapoints;
    for i in 0..ctx.data.datapoints().len() {
        point_buffer.push(ctx.data.datapoints()[i].clone());
        if point_buffer.len() >= graph_ratio {
            ctx.graph.push_stat(GraphDataPoint::from_datapoints(&point_buffer), max_datapoints);
            ctx.graph.update_timestamps();
            point_buffer.clear();
        }
    }
    ctx.graph.update_timestamps();
}
