use std::sync::{Arc, Mutex};

const NUM_TASKS: usize = 4;
const NUM_LOOP: usize = 100000;

#[tokio::main]
async fn main() -> Result<(), tokio::task::JoinError> {
    let val = Arc::new(Mutex::new(0));
    let mut v = Vec::new();
    for _ in 0..NUM_TASKS {
        let n = val.clone();
        v.push(tokio::spawn(async move {
            for _ in 0..NUM_LOOP {
                let mut n0 = n.lock().unwrap();
                *n0 += 1;
            }
        }));
    }

    for i in v {
        i.await?;
    }

    println!("COUNT = {} (exepected {})", *val.lock().unwrap(), NUM_TASKS * NUM_LOOP);
    Ok(())
}