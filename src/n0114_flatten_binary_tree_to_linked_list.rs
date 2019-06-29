/**
 * [114] Flatten Binary Tree to Linked List
 *
 * Given a binary tree, flatten it to a linked list in-place.
 * 
 * For example, given the following tree:
 * 
 * 
 *     1
 *    / \
 *   2   5
 *  / \   \
 * 3   4   6
 * 
 * 
 * The flattened tree should look like:
 * 
 * 
 * 1
 *  \
 *   2
 *    \
 *     3
 *      \
 *       4
 *        \
 *         5
 *          \
 *           6
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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(root) = root {
            Solution::flatten(&mut root.borrow().left.clone());
            Solution::flatten(&mut root.borrow().right.clone());
            let mut root= root.borrow_mut();
            if let Some(mut left) = root.left.as_ref().cloned() {
                loop {
                    if left.borrow().right.is_none() {
                        break;
                    }
                    let n = left.borrow().right.as_ref().cloned().unwrap();
                    left = n;
                }
                left.borrow_mut().right = root.right.take();
                root.right = root.left.take();
            };
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_114() {
        let mut root = tree![1,2,5,3,4,n,6];
        Solution::flatten(&mut root);
        assert_eq!(root, tree![1,n,2,n,3,n,4,n,5,n,6]);
    }
}
