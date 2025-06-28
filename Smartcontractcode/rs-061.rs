use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn sleep(ms: u64) -> JoinHandle<u32> {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(ms));
        1
    })
}

pub fn add(h1: JoinHandle<u32>, h2: JoinHandle<u32>) -> u32 {
    h1.join().unwrap() + h2.join().unwrap()
}