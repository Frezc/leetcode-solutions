/**
 * [129] Sum Root to Leaf Numbers
 *
 * Given a binary tree containing digits from 0-9 only, each root-to-leaf path could represent a number.
 * 
 * An example is the root-to-leaf path 1->2->3 which represents the number 123.
 * 
 * Find the total sum of all root-to-leaf numbers.
 * 
 * Note: A leaf is a node with no children.
 * 
 * Example:
 * 
 * 
 * Input: [1,2,3]
 *     1
 *    / \
 *   2   3
 * Output: 25
 * Explanation:
 * The root-to-leaf path 1->2 represents the number 12.
 * The root-to-leaf path 1->3 represents the number 13.
 * Therefore, sum = 12 + 13 = 25.
 * 
 * Example 2:
 * 
 * 
 * Input: [4,9,0,5,1]
 *     4
 *    / \
 *   9   0
 *  / \
 * 5   1
 * Output: 1026
 * Explanation:
 * The root-to-leaf path 4->9->5 represents the number 495.
 * The root-to-leaf path 4->9->1 represents the number 491.
 * The root-to-leaf path 4->0 represents the number 40.
 * Therefore, sum = 495 + 491 + 40 = 1026.
 * 
 */

/// DFS
/// If you use bottom-up algorithm, you should treat repeat 0 path special.
/// example:
///             1
///            /
///           0
///          / \
///         0   0
/// The 'zero' subtree should return [00, 00].
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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let mut result = vec![];
            sum_numbers(&mut result, 0, root);
            result.into_iter().sum()
        } else {
            0
        }
    }
}

fn sum_numbers(result: &mut Vec<i32>, prefix: i32, root: Rc<RefCell<TreeNode>>) {
    let next_prefix = prefix * 10 + root.borrow().val;
    if root.borrow().left.is_none() && root.borrow().right.is_none() {
        return result.push(next_prefix);
    }

    if let Some(node) = root.borrow().left.as_ref() {
        sum_numbers(result, next_prefix,Rc::clone(node));
    }

    if let Some(node) = root.borrow().right.as_ref() {
        sum_numbers(result, next_prefix,Rc::clone(node));
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_129() {
        assert_eq!(Solution::sum_numbers(tree![1,2,3]), 25);
        assert_eq!(Solution::sum_numbers(tree![4,9,0,5,1]), 1026);
        assert_eq!(Solution::sum_numbers(tree![5,3,2,7,0,6,null,null,null,0]), 6363);
    }
}
