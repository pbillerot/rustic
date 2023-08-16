use std::collections::HashMap;


pub fn default_str() -> String {
    "".to_string()
}
pub fn default_map() -> HashMap<String, String> {
    HashMap::new()
}
pub fn default_bool() -> bool {
    false
}
#[allow(dead_code)]
pub fn default_vec() -> Vec<String> {
    Vec::new()
}

// This returns a cloned HashMap
// because Rust uses the Clone-implementation on HashMap
pub fn do_clone_map<K: Clone, V: Clone>(data: &HashMap<K,V>) -> HashMap<K, V> {
    data.clone()
}
