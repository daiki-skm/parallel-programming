#![feature(asm)]

use std::sync::Arc;

mod stack;

const NUM_LOOP: usize = 1000000;
const NUM_THREADS: usize = 4;

use stack::Stack;

fn main() {
    let stack = Arc::new(Stack::<usize>::new());
    let mut v = Vec::new();

    for i in 0..NUM_THREADS {
        let stack0 = stack.clone();
        let t = std::thread::spawn(move || {
            if i & 1 == 0 {
                for j in 0..NUM_LOOP {
                    let k = i * NUM_LOOP + j;
                    stack0.get_mut().push(k);
                    println!("push: {}", k);
                }
                println!("finished push: #{}", i);
            } else {
                for _ in 0..NUM_LOOP {
                    loop {
                        if let Some(k) = stack0.get_mut().pop() {
                            println!("pop: {}", k);
                            break;
                        }
                    }
                }
                println!("finished pop: #{}", i);
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }

    assert!(stack.get().pop() == None);
}