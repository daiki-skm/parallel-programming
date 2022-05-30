use std::{sync::Arc, time};
use tokio::sync::Mutex;

const NUM_TASKS: usize = 8;

async fn lock_only(v: Arc<Mutex<u64>>) {
    let mut n = v.lock().await;
    *n += 1;
}

async fn lock_sleep(v: Arc<Mutex<u64>>) {
    let mut n = v.lock().await;
    let ten_secs = time::Duration::from_secs(10);
    tokio::time::sleep(ten_secs).await;
    *n += 1;
}

#[tokio::main]
async fn main() -> Result<(), tokio::task::JoinError> {
    let val = Arc::new(Mutex::new(0));
    let mut v = Vec::new();

    let t = tokio::spawn(lock_sleep(val.clone()));
    v.push(t);

    for _ in 0..NUM_TASKS {
        let t = tokio::spawn(lock_only(val.clone()));
        v.push(t);
    }

    for t in v {
        t.await?;
    }
    Ok(())
}