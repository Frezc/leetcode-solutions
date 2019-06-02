/**
 * [100] Same Tree
 *
 * Given two binary trees, write a function to check if they are the same or not.
 * 
 * Two binary trees are considered the same if they are structurally identical and the nodes have the same value.
 * 
 * Example 1:
 * 
 * 
 * Input:     1         1
 *           / \       / \
 *          2   3     2   3
 * 
 *         [1,2,3],   [1,2,3]
 * 
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input:     1         1
 *           /           \
 *          2             2
 * 
 *         [1,2],     [1,null,2]
 * 
 * Output: false
 * 
 * 
 * Example 3:
 * 
 * 
 * Input:     1         1
 *           / \       / \
 *          2   1     1   2
 * 
 *         [1,2,1],   [1,1,2]
 * 
 * Output: false
 * 
 * 
 */
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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        p == q
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_100() {
        assert_eq!(Solution::is_same_tree(tree![1,2,3], tree![1,2,3]), true);
        assert_eq!(Solution::is_same_tree(tree![1,2], tree![1,null,2]), false);
        assert_eq!(Solution::is_same_tree(tree![1,2,1], tree![1,1,2]), false);
    }
}
