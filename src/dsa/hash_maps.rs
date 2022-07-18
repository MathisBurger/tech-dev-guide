use std::collections::HashMap;

/// Usage example of a hash map
///
/// HashMaps have got random access to the memory.
/// Every value is associated with a specific key
/// The key is passed into a hash function with decides where to store the value.
/// Therefore, hashMaps can easily grow in size and do not have a fixed size
pub fn hash_map() {

    // Initialize the hashmap
    let mut hash_map: HashMap<String, String> = HashMap::new();

    // Inserts a the value "value" with the key "key" into the hashmap
    hash_map.insert("key".to_string(), "value".to_string());

    // Gets the value with the key "key" from the hashmap
    let value = hash_map.get("key");

    println!("{}", value.unwrap());
}