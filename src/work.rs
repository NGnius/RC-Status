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
        self.handles.push(crate::staticdata::start_worker());
    }

    pub fn stop(self) {
        for handle in self.handles {
            handle.join().expect("Joining stopping thread failed");
        }
    }
}