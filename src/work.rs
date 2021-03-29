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
    }

    pub fn stop(self) {
        for handle in self.handles {
            handle.join().expect("Joining stopping thread failed");
        }
    }
}