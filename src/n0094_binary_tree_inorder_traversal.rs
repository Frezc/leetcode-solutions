/**
 * [94] Binary Tree Inorder Traversal
 *
 * Given a binary tree, return the inorder traversal of its nodes' values.
 * 
 * Example:
 * 
 * 
 * Input: [1,null,2,3]
 *    1
 *     \
 *      2
 *     /
 *    3
 * 
 * Output: [1,3,2]
 * 
 * Follow up: Recursive solution is trivial, could you do it iteratively?
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = vec![];
        let mut left_read_stack = vec![];
        let mut result = vec![];
        if let Some(root_node) = root.as_ref() {
            stack.push(root_node.clone());
            while !stack.is_empty() || !left_read_stack.is_empty() {
                let node = stack.pop().unwrap();
                if let Some(left) = node.as_ref().borrow().left.as_ref() {
                    left_read_stack.push(node.clone());
                    stack.push(left.clone());
                    continue;
                }
                left_read_stack.push(node.clone());

                while stack.is_empty() && !left_read_stack.is_empty() {
                    let cur = left_read_stack.pop().unwrap();
                    result.push(cur.as_ref().borrow().val);
                    if let Some(right) = cur.as_ref().borrow().right.as_ref() {
                        stack.push(right.clone());
                    };
                }
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
    use crate::tree::TreeNode;

    #[test]
    fn test_94() {
        assert_eq!(Solution::inorder_traversal(tree![1,null,2,3]), vec![1,3,2]);
    }
}
