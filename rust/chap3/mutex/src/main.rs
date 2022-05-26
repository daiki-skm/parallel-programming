use std::sync::{Arc, Mutex};
use std::thread;

fn some_func(th: &str, lock: Arc<Mutex<u64>>) {
    loop {
        let mut val = lock.lock().unwrap();
        *val += 1;
        println!("{} = {}", th, *val);
    }
}

fn main() {
    let lock0 = Arc::new(Mutex::new(0));

    let lock1 = lock0.clone();

    let th0 = thread::spawn(move || {
        some_func("0", lock0);
    });

    let th1 = thread::spawn(move || {
        some_func("1", lock1);
    });

    th0.join().unwrap();
    th1.join().unwrap();
}