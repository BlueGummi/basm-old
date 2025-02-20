use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
pub static VARIABLE_MAP: Lazy<Mutex<HashMap<std::ops::Range<usize>, (usize, i64)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static LABEL_MAP: Lazy<Mutex<HashMap<std::ops::Range<usize>, (usize, i64)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

