pub fn start_cardlife_workers(handles: &mut Vec<std::thread::JoinHandle<()>>) {
    handles.push(super::server_list::start_worker());
}
