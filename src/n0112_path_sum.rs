/**
 * [112] Path Sum
 *
 * Given a binary tree and a sum, determine if the tree has a root-to-leaf path such that adding up all the values along the path equals the given sum.
 * 
 * Note: A leaf is a node with no children.
 * 
 * Example:
 * 
 * Given the below binary tree and sum = 22,
 * 
 * 
 *       5
 *      / \
 *     4   8
 *    /   / \
 *   11  13  4
 *  /  \      \
 * 7    2      1
 * 
 * 
 * return true, as there exist a root-to-leaf path 5->4->11->2 which sum is 22.
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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if let Some(node) = root {
            if node.borrow().val == sum {
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    return true;
                }
            }
            return Solution::has_path_sum(node.borrow().left.clone(), sum - node.borrow().val) ||
                Solution::has_path_sum(node.borrow().right.clone(), sum - node.borrow().val);
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_112() {
        assert_eq!(Solution::has_path_sum(tree![5,4,8,11,n,13,4,7,2,n,n,n,1], 22), true);
        assert_eq!(Solution::has_path_sum(tree![], 0), false);
        assert_eq!(Solution::has_path_sum(tree![1,-2,-3,1,3,-2,null,-1], -1), true);
    }
}
