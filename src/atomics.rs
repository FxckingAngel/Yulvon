use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;
use std::sync::Arc;

pub struct AtomicQueue<T: Copy + Default> {
    buffer: Vec<UnsafeCell<T>>,
    head: AtomicUsize,
    tail: AtomicUsize,
    size: usize,
}

unsafe impl<T: Copy + Default> Sync for AtomicQueue<T> {}
unsafe impl<T: Copy + Default> Send for AtomicQueue<T> {}

impl<T: Copy + Default> AtomicQueue<T> {
    pub fn new(size: usize) -> Self {
        let mut buffer = Vec::with_capacity(size);
        for _ in 0..size {
            buffer.push(UnsafeCell::new(T::default()));
        }
        Self {
            buffer,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            size,
        }
    }
    pub fn push(&self, value: T) -> bool {
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) % self.size;
        if next_tail == self.head.load(Ordering::Acquire) {
            return false; // full
        }
        unsafe { *self.buffer[tail].get() = value; }
        self.tail.store(next_tail, Ordering::Release);
        true
    }
    pub fn pop(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);
        if head == self.tail.load(Ordering::Acquire) {
            return None; // empty
        }
        let value = unsafe { *self.buffer[head].get() };
        self.head.store((head + 1) % self.size, Ordering::Release);
        Some(value)
    }
}

pub struct AtomicCounter {
    value: AtomicUsize,
}

impl AtomicCounter {
    pub fn new(val: usize) -> Self {
        Self { value: AtomicUsize::new(val) }
    }
    pub fn load(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }
    pub fn store(&self, val: usize) {
        self.value.store(val, Ordering::SeqCst)
    }
    pub fn fetch_add(&self, val: usize) -> usize {
        self.value.fetch_add(val, Ordering::SeqCst)
    }
    pub fn compare_and_swap(&self, current: usize, new: usize) -> usize {
        self.value.compare_and_swap(current, new, Ordering::SeqCst)
    }
}
