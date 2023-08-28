use std::collections::HashMap;


pub fn default_bool() -> bool {
    false
}
pub fn default_i32() -> i32 {
    0
}
// This returns a cloned HashMap
// because Rust uses the Clone-implementation on HashMap
pub fn do_clone_map<K: Clone, V: Clone>(data: &HashMap<K,V>) -> HashMap<K, V> {
    data.clone()
}
