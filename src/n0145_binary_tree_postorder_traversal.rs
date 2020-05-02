/**
 * [145] Binary Tree Postorder Traversal
 *
 * Given a binary tree, return the postorder traversal of its nodes' values.
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
 * Output: [3,2,1]
 * 
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
    /// use stack to simulate recursive
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = vec![];
        let mut result = vec![];
        if root.is_none() { return result; }
        stack.push(Item {
            node: root.unwrap(),
            need_check: true,
        });
        while let Some(Item { node: iter, need_check }) = stack.pop() {
            if need_check {
                stack.push(Item {
                    node: iter.clone(),
                    need_check: false
                });
                if let Some(node) = iter.borrow().right.as_ref() {
                    stack.push(Item {
                        node: node.clone(),
                        need_check: true
                    });
                }
                if let Some(node) = iter.borrow().left.as_ref() {
                    stack.push(Item {
                        node: node.clone(),
                        need_check: true
                    });
                }
            } else {
                result.push(iter.borrow().val);
            }
        }

        result
    }
}

struct Item {
    node: Rc<RefCell<TreeNode>>,
    need_check: bool,
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_145() {
        assert_eq!(Solution::postorder_traversal(tree![1,null,2,3]), vec![3,2,1]);
        assert_eq!(Solution::postorder_traversal(tree![]), vec![]);
    }
}
