use gloo_storage::{LocalStorage, Storage};

use crate::models::StoredResult;

const STORAGE_KEY: &str = "enneagram.latest_result";

pub fn save_result(result: &StoredResult) {
    let _ = LocalStorage::set(STORAGE_KEY, result);
}

pub fn load_result() -> Option<StoredResult> {
    LocalStorage::get(STORAGE_KEY).ok()
}
