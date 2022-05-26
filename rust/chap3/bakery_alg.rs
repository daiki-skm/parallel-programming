use std::ptr::{read_volatile, write_volatile}; // 1
use std::sync::atomic::{fence, Ordering}; // 2
use std::thread;

const NUM_THREADS: usize = 4;
const NUM_LOOP: usize = 100000;

// 3
macro_rules! read_mem {
    ($addr: expr) => { unsafe { read_volatile($addr) } };
}

macro_rules! write_mem {
    ($addr: expr, $val: expr) => { unsafe { write_volatile($addr, $val) } };
}


// 4
struct BakeryLock {
    entering: [bool; NUM_THREADS],
    tickets: [Option<u64>; NUM_THREADS],
}

impl BakeryLock {
    fn lock(&mut self, idx: usize) -> LockGuard {
        // 5
        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[idx], true);
        fence(Ordering::SeqCst);

        // 6
        let mut max = 0;
        for i in 0..NUM_THREADS {
            if let Some(t) = read_mem!(&self.tickets[i]) {
                max = max.max(t);
            }
        }

        // 7
        let ticket = max + 1;
        write_mem!(&mut self.tickets[idx], Some(ticket));

        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[idx], false); // 8
        fence(Ordering::SeqCst);

        // 9
        for i in 0..NUM_THREADS {
            if i == idx {
                continue;
            }

            while read_mem!(&self.entering[i]) {} // 10

            loop {
                // 11
                match read_mem!(&self.tickets[i]) {
                    Some(t) => {
                        if ticket < t || (ticket == t && idx < i) {
                            break;
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        }

        fence(Ordering::SeqCst);
        LockGuard { idx }
    }
}

// 12
struct LockGuard {
    idx: usize,
}

impl Drop for LockGuard {
    // 13
    fn drop(&mut self) {
        fence(Ordering::SeqCst);
        write_mem!(&mut LOCK.tickets[self.idx], None);
        fence(Ordering::SeqCst);
    }
}

// 14
static mut LOCK: BakeryLock = BakeryLock {
    entering: [false; NUM_THREADS],
    tickets: [None; NUM_THREADS],
};

static mut COUNT: u64 = 0;

fn main() {
    let mut v = Vec::new();
    for i in 0..NUM_THREADS {
        v.push(thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                let _lock = unsafe { LOCK.lock(i) };
                unsafe {
                    let c = read_volatile(&COUNT);
                    write_volatile(&mut COUNT, c + 1);
                }
            }
        }));
    }

    for th in v {
        th.join().unwrap();
    }

    println!("COUNT = {} (expected = {})", unsafe { COUNT }, NUM_THREADS * NUM_LOOP);
}