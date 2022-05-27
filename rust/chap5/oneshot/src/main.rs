use tokio::sync::oneshot;

async fn set_val_later(tx: oneshot::Sender<i32>) {
    let ten_secs = std::time::Duration::from_secs(10);
    tokio::time::sleep(ten_secs).await;
    if let Err(_) = tx.send(100) {
        println!("failed to send");
    }
}

#[tokio::main]
pub async fn main() {
    let (tx, rx) = oneshot::channel();
    tokio::spawn(set_val_later(tx));
    match rx.await {
        Ok(n) => println!("n = {}", n),
        Err(e) => {
            println!("failed to receive: {}", e);
            return;
        }
    }
}