use std::sync::Mutex;
use std::thread;

pub fn count() -> u32 {
    let counter: Mutex<u32> = Mutex::new(0);
    
    thread::scope(|scope| {
        scope.spawn(|| {
            let mut val = counter.lock().unwrap();
            *val += 1;
        });
        scope.spawn(|| {
            let mut val = counter.lock().unwrap();
            *val += 1;
        });
    });
    
    let val = *counter.lock().unwrap();
    val
}