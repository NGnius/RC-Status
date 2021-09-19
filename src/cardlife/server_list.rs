use std::thread::{spawn, sleep, JoinHandle};

use serde::{Deserialize, Serialize};
use chrono::prelude::{Utc};
use libfj::cardlife::{LiveAPI, LiveGameInfo};

const INDICATOR_NAME: &str = "Lobby";

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerItem {
    id: usize,
    name: String,
    max_players: usize,
    current_players: usize,
    ping: usize,
    pvp: bool,
    official: bool,
    region: String,
    mods: String,
    no_mods: bool,
    password_protected: bool,
}

impl From<LiveGameInfo> for ServerItem {
    fn from(other: LiveGameInfo) -> Self {
        Self {
            id: other.id,
            name: other.world_name,
            max_players: other.max_players,
            current_players: other.current_players,
            ping: other.ping,
            pvp: other.is_pvp,
            official: other.is_official,
            region: other.region.to_uppercase().to_string(),
            no_mods: other.mod_info.to_lowercase() == "{\"orderedmods\":[]}",
            mods: other.mod_info,
            password_protected: other.has_password
        }
    }
}

pub fn start_worker() -> JoinHandle<()> {
    //populate_graph_points();
    spawn(server_worker)
}

fn server_worker() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let email = crate::CONFIG.read().unwrap().cardlife_email.clone();
    let password = crate::CONFIG.read().unwrap().cardlife_password.clone();
    while ! (*crate::IS_STOPPING.read().unwrap()).clone() {
        let api_res = rt.block_on(LiveAPI::login_email(&email, &password));
        if let Ok(api) = api_res { // login succeeded
            // retrieve online servers
            let start = Utc::now();
            let result = rt.block_on(api.lobbies());
            let duration = ((Utc::now() - start).num_microseconds().unwrap() as f32)/1000.0;
            if let Ok(lobbies) = result { // lobbies query succeeded
                let mut ctx = super::CL_CONTEXT.write().unwrap();
                ctx.indicators.update(INDICATOR_NAME, true, duration, true);
                ctx.server_list.clear();
                // update with new lobbies
                for s in lobbies.games {
                    ctx.server_list.push(s.into());
                }
                println!("Updated CL lobbies successfully");
            } else {
                println!("Updated CL lobbies unsuccessfully: {}", result.err().unwrap());
                super::CL_CONTEXT.write().unwrap().indicators.update_error(INDICATOR_NAME, true, true);
            }
        } else {
            super::CL_CONTEXT.write().unwrap().indicators.update_error(INDICATOR_NAME, true, true);
            println!("Failed to login to Cardlife API, skipping lobby request: {}", api_res.err().unwrap());
        }
        // no API spam
        let dur = crate::CONFIG.read().unwrap().period_ms;
        sleep(std::time::Duration::from_millis(dur));
    }
}
