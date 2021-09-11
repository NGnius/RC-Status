pub struct Workers {
    handles: Vec<std::thread::JoinHandle<()>>,
}

impl Workers {
    pub fn new() -> Workers {
        Workers{
            handles: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.handles.push(crate::configuration::start_worker());
        self.handles.push(crate::persist::start_worker());
        self.handles.push(crate::staticdata::start_worker());
        self.handles.push(crate::maintenance::start_worker());
        self.handles.push(crate::ping::start_worker());
        self.handles.push(crate::graphing::start_worker());
        self.handles.push(crate::social_status::start_worker());
        self.handles.push(crate::payment_status::start_worker());
        self.handles.push(crate::leaderboard_status::start_worker());
        self.handles.push(crate::auth_status::start_worker());
        self.handles.push(crate::logs_status::start_worker());
        self.handles.push(crate::cdn_status::start_worker());
        self.handles.push(crate::graph_cleaner::start_worker());
    }

    pub fn stop(self) {
        for handle in self.handles {
            handle.join().expect("Joining stopping thread failed");
        }
    }
}
