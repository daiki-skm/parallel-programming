use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicUsize, Ordering, fence, AtomicBool};

pub const NUM_LOCK: usize = 8;

const MASK: usize = NUM_LOCK - 1;

pub struct FairLock<T> {
    waiting: Vec<AtomicBool>,
    lock: AtomicBool,
    turn: AtomicUsize,
    data: UnsafeCell<T>,
}

pub struct FairLockGuard<'a, T> {
    fair_lock: &'a FairLock<T>,
    idx: usize,
}

impl<T> FairLock<T> {
    pub fn new(v: T) -> Self {
        let mut vec = Vec::new();
        for _ in 0..NUM_LOCK {
            vec.push(AtomicBool::new(false));
        }

        FairLock {
            waiting: vec,
            lock: AtomicBool::new(false),
            turn: AtomicUsize::new(0),
            data: UnsafeCell::new(v),
        }
    }

    pub fn lock(&self, idx: usize) -> FairLockGuard<T> {
        assert!(idx < NUM_LOCK);

        self.waiting[idx].store(true, Ordering::Relaxed);
        loop {
            if !self.waiting[idx].load(Ordering::Relaxed) {
                break;
            }

            if !self.lock.load(Ordering::Relaxed) {
                if let Ok(_) = self.lock.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed) {
                    break;
                }
            }
        }
        fence(Ordering::Acquire);

        FairLockGuard {
            fair_lock: self,
            idx: idx,
        }
    }
}

impl<'a, T> Drop for FairLockGuard<'a, T> {
    fn drop(&mut self) {
        let fl = self.fair_lock;

        fl.waiting[self.idx].store(false, Ordering::Relaxed);

        let turn = fl.turn.load(Ordering::Relaxed);
        let next = if turn == self.idx {
            (turn + 1) & MASK
        } else {
            turn
        };

        if fl.waiting[next].load(Ordering::Relaxed) {
            fl.turn.store(next, Ordering::Relaxed);
            fl.waiting[next].store(false, Ordering::Relearse);
        } else {
            fl.turn.store((next + 1) & MASK, Ordering::Relaxed);
            fl.lock.store(false, Ordering::Relearse);
        }
    }
}

unsafe impl<T> Sync for FairLock<T> {}
unsafe impl<T> Send for FairLock<T> {}

impl <'a, T> Deref for FairLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.fair_lock.data.get() }
    }
}

impl <'a, T> DerefMut for FairLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.fair_lock.data.get() }
    }
}