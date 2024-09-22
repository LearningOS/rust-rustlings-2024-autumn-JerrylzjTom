/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let new_node = Box::new(TreeNode::new(value));
        match self.root {
            Some(ref mut node) => {
                BinarySearchTree::insert_into_node(node, new_node);
            }
            None => {
                self.root = Some(new_node);
            }
        }
    }

    fn insert_into_node(current: &mut Box<TreeNode<T>>, new_node: Box<TreeNode<T>>) {
        if new_node.value < current.value {
            match current.left {
                Some(ref mut left_child) => {
                    BinarySearchTree::insert_into_node(left_child, new_node);
                }
                None => {
                    current.left = Some(new_node);
                }
            }
        } else if new_node.value > current.value {
            match current.right {
                Some(ref mut right_child) => {
                    BinarySearchTree::insert_into_node(right_child, new_node);
                }
                None => {
                    current.right = Some(new_node);
                }
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        BinarySearchTree::search_in_node(&self.root, &value)
    }
    fn search_in_node(node: &Option<Box<TreeNode<T>>>, value: &T) -> bool {
        match node {
            Some(ref current_node) => {
                if &current_node.value == value {
                    true
                } else if value < &current_node.value {
                    BinarySearchTree::search_in_node(&current_node.left, value)
                } else {
                    BinarySearchTree::search_in_node(&current_node.right, value)
                }
            }
            None => false,
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


