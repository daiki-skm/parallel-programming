mod banker;

use banker::Banker;
use std::thread;

const NUM_LOOP: usize = 100000;

fn main() {
    let banker = Banker::<2, 2>::new([1,1], [[1,1], [1,1]]);
    let banker0 = banker.clone();

    let philosopher0 = thread::spawn(move || {
        for _ in 0..NUM_LOOP {
            while !banker0.take(0,0) {}
            while !banker0.take(0,1) {}

            println!("0: eating");

            banker0.release(0,0);
            banker0.release(0,1);
        }
    });

    let philosopher1 = thread::spawn(move || {
        for _ in 0..NUM_LOOP {
            while !banker0.take(1,1) {}
            while !banker0.take(1,0) {}

            println!("1: eating");

            banker0.release(1,1);
            banker0.release(1,0);
        }
    });

    philosopher0.join().unwrap();
    philosopher1.join().unwrap();
}