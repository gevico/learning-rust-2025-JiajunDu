/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT DONE
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
        //TODO
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
        } else {
            let mut node_ref = self.root.as_mut().unwrap();
            loop {
                if value < node_ref.value {
                    if node_ref.left.is_none() {
                        node_ref.left = Some(Box::new(TreeNode::new(value)));
                        break;
                    } else {
                        node_ref = node_ref.left.as_mut().unwrap();
                    }
                } else if value > node_ref.value {
                    if node_ref.right.is_none() {
                        node_ref.right = Some(Box::new(TreeNode::new(value)));
                        break;
                    } else {
                        node_ref = node_ref.right.as_mut().unwrap();
                    }
                } else {
                   break;
                }
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        if self.root.is_none() {
            return false;
        }
        let mut node_ref = self.root.as_ref().unwrap();
        loop {
            if value == node_ref.value {
                return true;
            } else if value < node_ref.value {
                if node_ref.left.is_none() {
                    break;
                }
                node_ref = node_ref.left.as_ref().unwrap();
            } else {
                if node_ref.right.is_none() {
                    break;
                }
                node_ref = node_ref.right.as_ref().unwrap();
            }
        }
        false
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


