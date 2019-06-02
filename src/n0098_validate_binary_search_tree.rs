/**
 * [98] Validate Binary Search Tree
 *
 * Given a binary tree, determine if it is a valid binary search tree (BST).
 * 
 * Assume a BST is defined as follows:
 * 
 * 
 * 	The left subtree of a node contains only nodes with keys less than the node's key.
 * 	The right subtree of a node contains only nodes with keys greater than the node's key.
 * 	Both the left and right subtrees must also be binary search trees.
 * 
 * 
 *  
 * 
 * Example 1:
 * 
 * 
 *     2
 *    / \
 *   1   3
 * 
 * Input: [2,1,3]
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 *     5
 *    / \
 *   1   4
 *      / \
 *     3   6
 * 
 * Input: [5,1,4,null,null,3,6]
 * Output: false
 * Explanation: The root node's value is 5 but its right child's value is 4.
 * 
 * 
 */
/// Same number is invalid in this problem.
/// And check the [lower, upper] bound for whole tree
pub struct Solution {}
use super::tree::{TreeNode, to_tree};

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_valid_bst(root, None, None)
    }
}

fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>, lower: Option<i32>, upper: Option<i32>) -> bool {
    if let Some(node) = root.as_ref() {
        let cur_val = node.borrow().val;
        if let Some(left) = node.borrow().left.as_ref() {
            let left = left.borrow().val;
            if left >= cur_val || lower.map_or(false, |l| l >= left) {
                return false;
            }
        }

        if let Some(right) = node.borrow().right.as_ref() {
            let right = right.borrow().val;
            if right <= cur_val || upper.map_or(false , |u| u <= right) {
                return false;
            }
        }

        if is_valid_bst(node.borrow().left.clone(), lower, Some(cur_val))
            && is_valid_bst(node.borrow().right.clone(), Some(cur_val), upper) {
            true
        } else {
            false
        }
    } else {
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_98() {
        assert_eq!(Solution::is_valid_bst(tree![2,1,3]), true);
        assert_eq!(Solution::is_valid_bst(tree![5,1,4,null,null,3,6]), false);
        assert_eq!(Solution::is_valid_bst(tree![1, 1]), false);
        assert_eq!(Solution::is_valid_bst(tree![1, 0, 1]), false);
        assert_eq!(Solution::is_valid_bst(tree![10,5,15,null,null,6,20]), false);
    }
}
