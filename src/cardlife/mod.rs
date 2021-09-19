use lazy_static::lazy_static;
use std::sync::RwLock;

mod context;
mod endpoint;
pub mod server_list;
mod work;

lazy_static! {
    pub static ref CL_CONTEXT: RwLock<context::CardlifeContext> = RwLock::new(context::CardlifeContext::new());
}

pub use endpoint::*;
pub use work::start_cardlife_workers;
