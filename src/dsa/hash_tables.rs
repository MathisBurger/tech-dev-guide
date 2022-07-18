use std::collections::LinkedList;

pub trait Hashable {
    /// Hashes the current object to
    /// a usize
    ///
    /// The hash function implemented for a specific type
    /// should be safe to prevent collisions and reduce the
    /// length of the linked list at the hashed index.
    fn hash(&self) -> usize;
}

pub struct HashTable {
    table: Vec<LinkedList<usize>>
}

impl HashTable {

    pub fn new() -> Self {
        HashTable {table: vec![]}
    }

    /// Inserts a new element into the hash table
    pub fn insert(&mut self, key: usize) {
        let hash = key.hash();
        let list = self.table.get(hash);
        let mut new_list = LinkedList::new();
        if list.is_some() {
            new_list = list.unwrap().clone();
        }
        new_list.push_back(key);
        self.table.insert(hash, new_list);
    }
}

impl Hashable for usize {
    fn hash(&self) -> usize {
        self.clone()
    }
}

/// A hashtable is a data structure combined of
/// arrays and linked lists.
///
/// The input value is being passed through a hash function
/// that generates a hash value. This hash value is used as
/// an index of an array. This array contains a linked list
/// that stores all values with the same hash as the index.
///
/// In theory a hash table has a lookup time from O(n/k) where
/// k is the size of the hash table
pub fn hash_table() {
    let mut table: HashTable = HashTable::new();
    table.insert(123);
}


