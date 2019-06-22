/**
 * [102] Binary Tree Level Order Traversal
 *
 * Given a binary tree, return the level order traversal of its nodes' values. (ie, from left to right, level by level).
 * 
 * 
 * For example:<br />
 * Given binary tree [3,9,20,null,null,15,7],<br />
 * 
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 * 
 * 
 * 
 * return its level order traversal as:<br />
 * 
 * [
 *   [3],
 *   [9,20],
 *   [15,7]
 * ]
 * 
 * 
 */

/// BFS level by level
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
use std::collections::vec_deque::VecDeque;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut queue = VecDeque::new();
        if let Some(root) = root {
            queue.push_back(root);
            while !queue.is_empty() {
                let mut temporary = vec![];
                let mut level = vec![];
                while !queue.is_empty() {
                    let node = queue.pop_front().unwrap();
                    level.push(node.borrow().val);
                    if let Some(left) = node.borrow().left.as_ref() {
                        temporary.push(left.clone());
                    }
                    if let Some(right) = node.borrow().right.as_ref() {
                        temporary.push(right.clone());
                    };
                }
                result.push(level);
                queue.extend(temporary);
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_102() {
        assert_eq!(Solution::level_order(tree![3,9,20,null,null,15,7]), vec2![
            [3],
            [9,20],
            [15,7]
        ]);
    }
}
