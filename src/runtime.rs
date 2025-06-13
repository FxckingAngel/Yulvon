use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    waker: Option<Waker>,
}

impl Task {
    pub fn new(fut: impl Future<Output = ()> + Send + 'static) -> Arc<Self> {
        Arc::new(Task {
            future: Mutex::new(Box::pin(fut)),
            waker: None,
        })
    }
}

pub struct Executor {
    queue: Mutex<VecDeque<Arc<Task>>>,
}

impl Executor {
    pub fn new() -> Self {
        Self { queue: Mutex::new(VecDeque::new()) }
    }
    pub fn spawn(&self, task: Arc<Task>) {
        self.queue.lock().unwrap().push_back(task);
    }
    pub fn run(&self) {
        while let Some(task) = self.queue.lock().unwrap().pop_front() {
            let waker = futures::task::noop_waker();
            let mut cx = Context::from_waker(&waker);
            let mut fut = task.future.lock().unwrap();
            let _ = fut.as_mut().poll(&mut cx);
        }
    }
}

pub fn spawn<F>(fut: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    let exec = EXECUTOR.with(|e| e.clone());
    exec.spawn(Task::new(fut));
}

thread_local! {
    static EXECUTOR: Arc<Executor> = Arc::new(Executor::new());
}

pub fn run_executor() {
    EXECUTOR.with(|e| e.run());
}
