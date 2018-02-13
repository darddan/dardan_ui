use std::sync::{Arc, RwLock};

pub type UiCell<T> = Arc<RwLock<T>>;

pub fn new_ui_cell<T>(val: T) -> UiCell<T> {
    Arc::new(RwLock::new(val))
}
