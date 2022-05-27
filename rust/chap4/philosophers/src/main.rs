use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let c0 = Arc::new(Mutex::new(0));
    let c1 = Arc::new(Mutex::new(0));

    let c0_p0 = c0.clone();
    let c1_p0 = c1.clone();

    let p0 = thread::spawn(move || {
        for i in 0..1000 {
            let _n1 = c0_p0.lock().unwrap();
            let _n2 = c1_p0.lock().unwrap();
            println!("0: eating, i = {}", i);
        }
    });

    let p1 = thread::spawn(move || {
        for i in 0..1000 {
            let _n1 = c1.lock().unwrap();
            let _n2 = c0.lock().unwrap();
            println!("1: eating, i = {}", i);
        }
    });

    p0.join().unwrap();
    p1.join().unwrap();
}