use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub fn send_and_sync() -> (Arc<Mutex<Vec<u32>>>, JoinHandle<()>) {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let clone = Arc::clone(&data);

    let handle = thread::spawn(move || {
        clone.lock().unwrap().push(4);
    });

    (data, handle)
}