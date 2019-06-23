/**
 * [110] Balanced Binary Tree
 *
 * Given a binary tree, determine if it is height-balanced.
 * 
 * For this problem, a height-balanced binary tree is defined as:
 * 
 * <blockquote>
 * a binary tree in which the depth of the two subtrees of every node never differ by more than 1.
 * </blockquote>
 * 
 * Example 1:
 * 
 * Given the following tree [3,9,20,null,null,15,7]:
 * 
 * 
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 * 
 * Return true.<br />
 * <br />
 * Example 2:
 * 
 * Given the following tree [1,2,2,3,3,null,null,4,4]:
 * 
 * 
 *        1
 *       / \
 *      2   2
 *     / \
 *    3   3
 *   / \
 *  4   4
 * 
 * 
 * Return false.
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
    pub fn max_depth_and_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
        if let Some(root) = root {
            let left = Solution::max_depth_and_balanced(root.borrow().left.clone());
            let right = Solution::max_depth_and_balanced(root.borrow().right.clone());
            let max_depth = left.0.max(right.0) + 1;
            let balanced = left.1 && right.1 && (left.0 - right.0).abs() < 2;
            return (max_depth, balanced);
        }
        (0, true)
    }
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Solution::max_depth_and_balanced(root).1;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_110() {
        assert_eq!(Solution::is_balanced(tree![3,9,20,null,null,15,7]), true);
        assert_eq!(Solution::is_balanced(tree![1,2,2,3,3,null,null,4,4]), false);
        assert_eq!(Solution::is_balanced(tree![1,2,2,3,null,null,3,4,null,null,4]), false);
    }
}
