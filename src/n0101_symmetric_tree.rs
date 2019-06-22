/**
 * [101] Symmetric Tree
 *
 * Given a binary tree, check whether it is a mirror of itself (ie, symmetric around its center).
 * 
 * For example, this binary tree [1,2,2,3,4,4,3] is symmetric:
 * 
 * 
 *     1
 *    / \
 *   2   2
 *  / \ / \
 * 3  4 4  3
 * 
 * 
 *  
 * 
 * But the following [1,2,2,null,3,null,3] is not:
 * 
 * 
 *     1
 *    / \
 *   2   2
 *    \   \
 *    3    3
 * 
 * 
 *  
 * 
 * Note:<br />
 * Bonus points if you could solve it both recursively and iteratively.
 * 
 */
/// Recursive solution:
/// 1. Suppose recursive function "solute(node1, node2)", put left & right nodes of root to input.
/// 2. In recursive function, check node1.val === node2.val
///     & solute(node1.left, node2.right) & solute(node1.right, node2.left)
///
/// Iterative solution:
/// 1. Suppose we have a queue to save iterate nodes, push left & right nodes of root into it.
/// 2. Pop top 2 elements and assert that their val are equal.
/// 3. Push follow elements: node1.left, node2.right, node1.right, node2.left to queue.
/// 4. Repeat 2, 3 until queue is empty.
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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            return is_symmetric(root.borrow().left.clone(), root.borrow().right.clone());
        }
        true
    }
}

fn is_symmetric(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if left.is_none() && right.is_none() {
        return true;
    }

    if let Some(left) = left {
        if let Some(right) = right {
            if left.borrow().val != right.borrow().val {
                return false;
            }
            return is_symmetric(left.borrow().left.clone(), right.borrow().right.clone())
                && is_symmetric(left.borrow().right.clone(), right.borrow().left.clone());
        }
    }
    return false
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_101() {
        assert_eq!(Solution::is_symmetric(tree![1,2,2,3,4,4,3]), true);
        assert_eq!(Solution::is_symmetric(tree![1,2,2,null,3,null,3]), false);
    }
}
