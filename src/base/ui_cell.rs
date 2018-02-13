use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub type UiCell<T> = Arc<RwLock<T>>;

pub fn new_ui_cell<T>(val: T) -> UiCell<T> {
    Arc::new(RwLock::new(val))
}
/*
pub struct UiCell <T> {
    val : Arc<RwLock<T>>,
}

impl <'a, T> UiCell <T> {
    pub fn new(val: T) -> Self {
        UiCell {
            val: Arc::new(RwLock::new(val)),
        }
    }

    pub fn read(&self) -> RwLockReadGuard<T> {
        self.val.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<T> {
        self.val.write().unwrap()
    }
}

impl<'a, T> Clone for UiCell<T> {
    fn clone(&self) -> Self {
        UiCell {
            val: Arc::clone(&self.val),
        }
    }
}
*/