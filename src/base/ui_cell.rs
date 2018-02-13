use std::sync::{Arc, Mutex, MutexGuard};

pub struct UiCell <T> {
    val : Arc<Mutex<T>>,
}

impl <T> UiCell <T> {
    pub fn new(val: T) -> Self {
        UiCell {
            val: Arc::new(Mutex::new(val)),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.val.lock().unwrap()
    }
}

impl<T> Clone for UiCell<T> {
    fn clone(&self) -> Self {
        UiCell {
            val: Arc::clone(&self.val),
        }
    }
}
