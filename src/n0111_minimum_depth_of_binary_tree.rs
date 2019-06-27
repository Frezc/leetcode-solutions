/**
 * [111] Minimum Depth of Binary Tree
 *
 * Given a binary tree, find its minimum depth.
 * 
 * The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
 * 
 * Note: A leaf is a node with no children.
 * 
 * Example:
 * 
 * Given binary tree [3,9,20,null,null,15,7],
 * 
 * 
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 * 
 * return its minimum depth = 2.
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
use std::collections::vec_deque::VecDeque;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut result = 0;
        if let Some(node) = root {
            queue.push_back(node);
            while !queue.is_empty() {
                result += 1;
                let mut temp = vec![];
                while let Some(node) = queue.pop_front() {
                    let mut left_empty = false;
                    if let Some(left) = node.borrow().left.as_ref() {
                        temp.push(Rc::clone(left));
                    } else {
                        left_empty = true;
                    }

                    if let Some(right) = node.borrow().right.as_ref() {
                        temp.push(Rc::clone(right));
                    } else {
                        if left_empty {
                            return result;
                        }
                    }
                }
                queue.extend(temp);
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
    fn test_111() {
        assert_eq!(Solution::min_depth(tree![3,9,20,null,null,15,7]), 2);
        assert_eq!(Solution::min_depth(tree![1,2]), 2);
    }
}
