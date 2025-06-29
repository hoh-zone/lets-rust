use std::sync::Arc;
// use std::sync::Mutex;
use tokio::join;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

pub async fn boil_pasta(logger: Arc<Mutex<Vec<String>>>) {
    sleep(Duration::from_millis(100)).await;
    logger.lock().await.push("pasta is ready".into());
}

pub async fn make_tomato_sauce(logger: Arc<Mutex<Vec<String>>>) {
    sleep(Duration::from_millis(50)).await;
    logger.lock().await.push("tomato sauce is ready".into());
}

pub async fn make_pasta(logger: Arc<Mutex<Vec<String>>>) {
    join!(
        boil_pasta(logger.clone()),
        make_tomato_sauce(logger.clone())
    );

    logger.lock().await.push("tomato pasta is ready".into());
}