use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

pub fn make_and_send() -> Receiver<String> {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx.send("hello".to_string()).unwrap();
        tx.send("rust".to_string()).unwrap();
    });
    rx
}