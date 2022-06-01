use std::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, Ordering};

struct Node<T> {
    next: AtomicPtr<Node<T>>,
    data: T,
}

pub struct StackBad<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> StackBad<T> {
    pub fn new() -> Self {
        StackBad {
            head: AtomicPtr::new(null_mut()),
        }
    }

    pub fn push(&self, v: T) {
        let node = Box::new(Node {
            next: AtomicPtr::new(null_mut()),
            data: v,
        });

        let ptr = Box::into_raw(node);

        unsafe {
            loop {
                let head = self.head.load(Ordering::Relaxed);
                (*ptr).next.store(head, Ordering::Relaxed);

                if let Ok(_) = self.head.compare_exchange_weak(head, ptr, Ordering::Release, Ordering::Relaxed) {
                    break;
                }
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        unsafe {
            loop {
                let head = self.head.load(Ordering::Relaxed);
                if head == null_mut() {
                    return None;
                }

                let next = (*head).next.load(Ordering::Relaxed);

                if let Ok(_) = self.head.compare_exchange_weak(head, next, Ordering::Acquire, Ordering::Relaxed) {
                    let h = Box::from_raw(head);
                    return Some((*h).data);
                }
            }
        }
    }
}

impl<T> Drop for StackBad<T> {
    fn drop(&mut self) {
        let mut node = self.head.load(Ordering::Relaxed);
        while node != null_mut() {
            let n = unsafe {
                Box::from_raw(node)
            };
            node = n.next.load(Ordering::Relaxed);
        }
    }
}