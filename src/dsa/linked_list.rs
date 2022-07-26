use std::collections::LinkedList;

/// A linked list is a list that contains multiple values that are
/// linked to each other. Each element of a singly linked list contains
/// a link to the next element in the list.
/// There are four key types:
/// - Singly linked lists
/// - Doubly linked lists
/// - Circular singly linked lists
/// - Circular doubly linked lists
///
/// Lookup: O(n)
pub fn linked_lists() {

    let mut linked_list = LinkedList::new();
    linked_list.push_back("element 1");
    linked_list.push_back("element 2");
    println!("{:?}", linked_list);
}