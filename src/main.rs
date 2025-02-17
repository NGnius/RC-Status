#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use lazy_static::lazy_static;
use std::sync::RwLock;

mod root;
mod context;
mod work;
mod staticdata;
mod persist;
mod configuration;
mod maintenance;
mod ping;
mod graphing;
mod status;
mod social_status;
mod payment_status;
mod leaderboard_status;
mod logs_status;
mod auth_status;
mod cdn_status;
mod graph_cleaner;
mod raw;
mod cardlife;

lazy_static! {
    pub static ref CONTEXT: RwLock<context::IndexContext> = RwLock::new(context::IndexContext::new());
    pub static ref IS_STOPPING: RwLock<bool> = RwLock::new(false);
    pub static ref CONFIG: RwLock<configuration::Config> = RwLock::new(configuration::Config::new());
}

fn main() {
    let mut workers = work::Workers::new();
    workers.start();
    rocket::ignite().mount("/",
        routes![
            root::index,
            raw::static_json,
            raw::robots,
            cardlife::cardlife,
            ]
        )
        .mount("/static", rocket_contrib::serve::StaticFiles::from("./static"))
        .attach(rocket_contrib::templates::Template::fairing())
        .launch();
    *IS_STOPPING.write().unwrap() = true;
    workers.stop();
}
