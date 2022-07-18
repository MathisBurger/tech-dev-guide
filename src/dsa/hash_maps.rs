use std::collections::HashMap;

/// Usage example of a hash map
pub fn hash_map() {

    // Initialize the hashmap
    let mut hash_map: HashMap<String, String> = HashMap::new();

    // Inserts a the value "value" with the key "key" into the hashmap
    hash_map.insert("key".to_string(), "value".to_string());

    // Gets the value with the key "key" from the hashmap
    let value = hash_map.get("key");

    println!("{}", value.unwrap());
}