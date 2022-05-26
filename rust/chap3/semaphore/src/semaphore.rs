use std::sync::{Condvar, Mutex};
use std::convert::TryInto;

pub struct Semaphore {
    mutex: Mutex<usize>,
    cond: Condvar,
    max: isize,
}

impl Semaphore {
    pub fn new(max: isize) -> Self {
        Semaphore {
            mutex: Mutex::new(0),
            cond: Condvar::new(),
            max,
        }
    }

    pub fn wait(&self) {
        let mut count = self.mutex.lock().unwrap();
        while *count >= self.max.try_into().unwrap() {
            count = self.cond.wait(count).unwrap();
        }
        *count += 1;
    }

    pub fn post(&self) {
        let mut count = self.mutex.lock().unwrap();
        *count -= 1;
        if *count <= self.max.try_into().unwrap() {
            self.cond.notify_one();
        }
    }
}
