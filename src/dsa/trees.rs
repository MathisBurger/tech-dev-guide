use std::rc::Rc;

/// Trees are data types that are nested.
/// A tree has multiple nodes. Every note has multiple
/// children, but only one parent node. The only exception is
/// the root node. It does not have a parent node, because it is the first
/// level node.
///
/// The most common tree is a binary tree.
/// It sorts the data after a specific schema and therefore has quick
/// lookup and insert times.
///
/// The most common way traversing a tree is in-order. In-Order means
/// displaying the tree from the smallest to the highest value.
/// Other traversing types are pre-order and post-order
///
/// Furthermore, there are two types of binary trees.
/// Unbalanced trees act more like a linked list while
/// balanced trees are better organized.
pub struct TreeNode {
    left: Rc<Option<Box<TreeNode>>>,
    right: Rc<Option<Box<TreeNode>>>,
    data: i32,
}

/// This implementation does not work
/// It is implemented the simple way just to
/// avoid complexity.
impl TreeNode {

    /// Creates a new parent node
    pub fn new(data: i32) -> Self {
        TreeNode {
            data,
            left: Rc::new(None),
            right: Rc::new(None)
        }
    }

    /// Inserts a new element into the tree
    pub fn insert(&mut self, data: i32) {
        if data < self.data {
            let left = Rc::clone(&self.left);
            if left.is_some() {
                let mut left_mut = Rc::clone(&self.left);
                left_mut.unwrap().insert(data);
            } else {
                self.left = Rc::new(Some(Box::new(TreeNode {data, left: Rc::new(None), right: Rc::new(None)})));
            }
        } else {
            let right = Rc::clone(&self.right);
            if right.is_some() {
                let mut right = Rc::clone(&self.right);
                right.unwrap().insert(data);
            } else {
                self.right = Rc::new(Some(Box::new(TreeNode {data, left: Rc::new(None), right: Rc::new(None)})));
            }
        }
    }

    /// Checks if the tree contains a value
    pub fn contains(&mut self, data: i32) -> bool {
        if self.data == data {
            return true;
        }
        if data < self.data {
            if Rc::clone(&self.left).is_some() {
                if Rc::clone(&self.left).unwrap().data == data {
                    return true;
                } else {
                    let mut left_mut = Rc::clone(&self.left);
                    return left_mut.unwrap().contains(data);
                }
            }
            return false;
        } else {
            if Rc::clone(&self.right).is_some() {
                if Rc::clone(&self.right).as_ref().unwrap().data == data {
                    return true;
                } else {
                    let mut right_mut  = Rc::clone(&self.right);
                    return right_mut.unwrap().contains(data);
                }
            }
            return false;
        }
    }
}

pub fn tree() {
    let mut tree = TreeNode::new(20);
    tree.insert(12);
    tree.insert(34);
    tree.insert(25);
    tree.insert(35);

    println!("{}", tree.contains(1));
    println!("{}", tree.contains(25));
}