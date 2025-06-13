use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::cell::UnsafeCell;

const MAX_TASKS: usize = 1024;

pub struct RtTask {
    pub func: fn(),
}

pub struct RtScheduler {
    queue: Arc<RtRingBuffer>,
}

struct RtRingBuffer {
    buffer: [UnsafeCell<Option<RtTask>>; MAX_TASKS],
    head: AtomicUsize,
    tail: AtomicUsize,
}

unsafe impl Sync for RtRingBuffer {}
unsafe impl Send for RtRingBuffer {}

impl RtRingBuffer {
    fn new() -> Self {
        Self {
            buffer: unsafe { std::mem::zeroed() },
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }
    fn push(&self, task: RtTask) -> bool {
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) % MAX_TASKS;
        if next_tail == self.head.load(Ordering::Acquire) {
            return false; // full
        }
        unsafe { *self.buffer[tail].get() = Some(task); }
        self.tail.store(next_tail, Ordering::Release);
        true
    }
    fn pop(&self) -> Option<RtTask> {
        let head = self.head.load(Ordering::Relaxed);
        if head == self.tail.load(Ordering::Acquire) {
            return None; // empty
        }
        let task = unsafe { (*self.buffer[head].get()).take() };
        self.head.store((head + 1) % MAX_TASKS, Ordering::Release);
        task
    }
}

impl RtScheduler {
    pub fn new() -> Self {
        Self { queue: Arc::new(RtRingBuffer::new()) }
    }
    pub fn schedule(&self, func: fn()) -> bool {
        self.queue.push(RtTask { func })
    }
    pub fn run(&self) {
        while let Some(task) = self.queue.pop() {
            (task.func)();
        }
    }
}
