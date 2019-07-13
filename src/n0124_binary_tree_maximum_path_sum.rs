/**
 * [124] Binary Tree Maximum Path Sum
 *
 * Given a non-empty binary tree, find the maximum path sum.
 * 
 * For this problem, a path is defined as any sequence of nodes from some starting node to any node in the tree along the parent-child connections. The path must contain at least one node and does not need to go through the root.
 * 
 * Example 1:
 * 
 * 
 * Input: [1,2,3]
 * 
 *        1
 *       / \
 *      2   3
 * 
 * Output: 6
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [-10,9,20,null,null,15,7]
 * 
 *    -10
 *    / \
 *   9  20
 *     /  \
 *    15   7
 * 
 * Output: 42
 * 
 * 
 */
/// For every nodes, we should consider two types of path:
/// 1. Left children to right children
/// 2. Parent to children(left or right)
///
/// For example
/// Input: [-10,9,20,null,null,15,7]
///
///    -10
///    / \
///   9  20 <-- input node
///     /  \
///    15   7
///
/// Get paths of above two types:
/// 1. 15 - 20 - 7
/// 2. -10 - 20 - 15 (15 > 7)
///
/// We name first type disconnected (disconnected to parent),
/// and the other connected (connected to parent).
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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let (disconnected, connected) = max_child(root);
            return disconnected.max(connected);
        }

        0
    }
}

fn max_child(root: Rc<RefCell<TreeNode>>) -> (i32, i32) {
    if root.borrow().left.is_none() && root.borrow().right.is_none() {
        return (root.borrow().val, root.borrow().val);
    }

    // storage the max disconnected path sum.
    // Also consider negative nodes like [-2, -1, -3]. It just save the max value.
    let mut max_disconnected: i32 = std::i32::MIN;
    let mut left_connected: i32 = 0;
    let mut right_connected: i32 = 0;
    if let Some(node) = root.borrow().left.as_ref() {
        let (disconnected, connected) = max_child(node.clone());
        // For disconnected path, just pick max, because it's completion of computed
        max_disconnected = max_disconnected.max(disconnected);
        // For connected path, we only use non-negative children.
        left_connected = connected.max(0);
    }

    if let Some(node) = root.borrow().right.as_ref() {
        let (disconnected, connected) = max_child(node.clone());
        max_disconnected = max_disconnected.max(disconnected);
        right_connected = connected.max(0);
    }

    let result = (
        // Calculate disconnected path
        max_disconnected.max(left_connected + right_connected + root.borrow().val),
        // Calculate connected path
        left_connected.max(right_connected) + root.borrow().val
    );

    result
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_124() {
        assert_eq!(Solution::max_path_sum(tree![1,2,3]), 6);
        assert_eq!(Solution::max_path_sum(tree![-10,9,20,null,null,15,7]), 42);
        assert_eq!(Solution::max_path_sum(tree![-2, -1]), -1);
        assert_eq!(Solution::max_path_sum(tree![9,6,-3,null,null,-6,2,null,null,2,null,-6,-6,-6]), 16);
    }
}
