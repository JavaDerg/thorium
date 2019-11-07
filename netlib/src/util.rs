use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_time() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}