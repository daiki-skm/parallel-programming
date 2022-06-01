use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicUsize, Ordering, fence};

pub struct Ticketlock<T> {
    ticket: AtomicUsize,
    turn: AtomicUsize,
    data: UnsafeCell<T>,
}

pub struct TicketlockGuard<'a, T> {
    ticket_lock: &'a Ticketlock<T>,
}

impl<T> Ticketlock<T> {
    pub fn new(v: T) -> Self {
        Ticketlock {
            ticket: AtomicUsize::new(0),
            turn: AtomicUsize::new(0),
            data: UnsafeCell::new(v),
        }
    }

    pub fn lock(&self) -> TicketlockGuard<T> {
        let t = self.ticket.fetch_add(1, Ordering::Relaxed);
        while self.turn.load(Ordering::Relaxed) != t {}
        fence(Ordering::Acquire);

        TicketlockGuard {
            ticket_lock: self
        }
    }
}

impl<'a, T> Drop for TicketlockGuard<'a, T> {
    fn drop(&mut self) {
        self.ticket_lock.turn.fetch_add(1, Ordering::Release);
    }
}

unsafe impl<T> Sync for Ticketlock<T> {}
unsafe impl<T> Send for Ticketlock<T> {}

impl <'a, T> Deref for TicketlockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ticket_lock.data.get() }
    }
}

impl <'a, T> DerefMut for TicketlockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ticket_lock.data.get() }
    }
}