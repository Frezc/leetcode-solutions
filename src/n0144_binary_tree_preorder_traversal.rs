/**
 * [144] Binary Tree Preorder Traversal
 *
 * Given a binary tree, return the preorder traversal of its nodes' values.
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
 * Output: [1,2,3]
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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if let Some(node) = root {
            traversal(node, &mut res);
        }
        res
    }
}

fn traversal(node: Rc<RefCell<TreeNode>>, res: &mut Vec<i32>) {
    res.push(node.borrow().val);
    if let Some(node) = node.borrow().left.as_ref() {
        traversal(node.clone(), res);
    }

    if let Some(node) = node.borrow().right.as_ref() {
        traversal(node.clone(), res);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_144() {
        assert_eq!(Solution::preorder_traversal(tree![1,null,2,3]), vec![1,2,3]);
        assert_eq!(Solution::preorder_traversal(tree![]), vec![]);
    }
}
