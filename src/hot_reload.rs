use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use libloading::{Library, Symbol};
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};

pub struct HotReloader {
    lib_path: PathBuf,
    lib: Arc<Mutex<Option<Library>>>,
}

impl HotReloader {
    pub fn new(lib_path: PathBuf) -> Self {
        Self {
            lib_path,
            lib: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(&self, symbol_name: &'static str) {
        let lib_path = self.lib_path.clone();
        let lib_arc = self.lib.clone();
        thread::spawn(move || {
            let (tx, rx) = std::sync::mpsc::channel();
            let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
            watcher.watch(&lib_path, RecursiveMode::NonRecursive).unwrap();
            loop {
                match rx.recv() {
                    Ok(DebouncedEvent::Write(_)) | Ok(DebouncedEvent::Create(_)) => {
                        let mut lib_guard = lib_arc.lock().unwrap();
                        *lib_guard = Library::new(&lib_path).ok();
                        println!("[hot-reload] Reloaded library: {:?}", lib_path);
                    }
                    _ => {}
                }
            }
        });
    }

    pub fn call<T, F>(&self, symbol_name: &'static str, f: F) -> Option<T>
    where
        F: FnOnce(Symbol<T>) -> T,
        T: Copy,
    {
        let lib_guard = self.lib.lock().unwrap();
        if let Some(ref lib) = *lib_guard {
            unsafe {
                if let Ok(symbol) = lib.get::<T>(symbol_name.as_bytes()) {
                    Some(f(symbol))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}
