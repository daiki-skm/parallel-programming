use std::sync::{Arc, Mutex};

fn main() {
    let lock0 = Arc::new(Mutex::new(0));
    let lock1 = lock0.clone();

    let a = lock0.lock().unwrap();
    let b = lock1.lock().unwrap();
    println!("a: {}", a);
    println!("b: {}", b);
}