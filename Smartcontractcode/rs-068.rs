use std::future::Future;
use tokio::time::{sleep, Duration};
use tokio::{join, select};

pub async fn get_with_timeout<F: Future<Output = u32>>(f: F, wait_time: u64) -> Option<u32> {
    select! {
        val = f => {
            Some(val)
        }
        _ = sleep(Duration::from_millis(wait_time)) => {
            None
        }
    }
}

pub async fn get_many_with_timeout<F: Future<Output = u32>>(
    f0: F,
    f1: F,
    f2: F,
    wait_time: u64,
) -> [Option<u32>; 3] {
    let (res0, res1, res2) = join!(
    get_with_timeout(f0, wait_time),
    get_with_timeout(f1, wait_time),
    get_with_timeout(f2, wait_time),
    );
    [res0, res1, res2]
}