use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
// Where it is            line   val
type Vtable = Lazy<Mutex<HashMap<std::ops::Range<usize>, (usize, i64)>>>;
type Ltable = Lazy<Mutex<HashMap<std::ops::Range<usize>, (usize, usize)>>>;

pub static VARIABLE_MAP: Vtable = Lazy::new(|| Mutex::new(HashMap::new()));

pub static LABEL_MAP: Ltable = Lazy::new(|| Mutex::new(HashMap::new()));
