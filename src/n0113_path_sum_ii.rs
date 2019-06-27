/**
 * [113] Path Sum II
 *
 * Given a binary tree and a sum, find all root-to-leaf paths where each path's sum equals the given sum.
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
 *  /  \    / \
 * 7    2  5   1
 * 
 * 
 * Return:
 * 
 * 
 * [
 *    [5,4,11,2],
 *    [5,8,4,5]
 * ]
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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        if let Some(node) = root {
            if node.borrow().val == sum {
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    return vec![vec![node.borrow().val]];
                }
            }

            let val = node.borrow().val;
            let mut result = [Solution::path_sum(node.borrow().left.clone(), sum - val),
                Solution::path_sum(node.borrow().right.clone(), sum - val)].concat();
            result.iter_mut().for_each(|path| path.insert(0, val));
            return result;
        }

        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_113() {
        assert_eq!(Solution::path_sum(tree![5,4,8,11,n,13,4,7,2,n,n,5,1], 22), vec2![
           [5,4,11,2],
           [5,8,4,5]
        ]);
        assert_eq!(Solution::path_sum(tree![], 0), vec![vec![0i32;0];0]);
        assert_eq!(Solution::path_sum(tree![1,-2,-3,1,3,-2,null,-1], -1), vec2![[1,-2,1,-1]]);
    }
}
