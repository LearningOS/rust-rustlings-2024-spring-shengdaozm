/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/
// I AM NOT DONE
//use std::cmp::Ordering;
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
        if self.root.is_none() { //没有初始化，就直接返回了
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        }
        let mut loop_node=self.root; //取到树里面的节点
        loop {
            if loop_node.is_none() {
                loop_node = Some(Box::new(TreeNode::new(value)));
                break;
            }
            if value < loop_node.unwrap().value {
                loop_node = loop_node.unwrap().left;
            } else if value > loop_node.unwrap().value {
                loop_node = loop_node.unwrap().right;
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        if self.root.is_none() { //空树
            return false;
        }
        let mut loop_node = self.root; //取出节点，直接循环
        loop {
            if loop_node.is_none() { //当前节点是空的
                return false;
            }
            if value == loop_node.unwrap().value {
                return true;
            }
            if value < loop_node.unwrap().value {
                loop_node = loop_node.unwrap().left;
            } else if value > loop_node.unwrap().value {
                loop_node = loop_node.unwrap().right;
            }
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


