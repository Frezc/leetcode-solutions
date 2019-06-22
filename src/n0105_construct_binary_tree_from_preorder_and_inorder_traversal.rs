/**
 * [105] Construct Binary Tree from Preorder and Inorder Traversal
 *
 * Given preorder and inorder traversal of a tree, construct the binary tree.
 * 
 * Note:<br />
 * You may assume that duplicates do not exist in the tree.
 * 
 * For example, given
 * 
 * 
 * preorder = [3,9,20,15,7]
 * inorder = [9,3,15,20,7]
 * 
 * Return the following binary tree:
 * 
 * 
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.len() == 0 {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
        if inorder.len() > 1 {
            let inorders: Vec<&[i32]> = inorder.split(|&n| n == preorder[0]).collect();
            let (l_p, r_p) = &preorder[1..].split_at(inorders[0].len());
            root.borrow_mut().left = Solution::build_tree(l_p.to_vec(), inorders.get(0).unwrap().to_vec());
            root.borrow_mut().right = Solution::build_tree(r_p.to_vec(), inorders.get(1).unwrap().to_vec());
        }

        Some(root)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_105() {
        assert_eq!(Solution::build_tree(vec![3,9,20,15,7], vec![9,3,15,20,7]), tree![3,9,20,n,n,15,7])
    }
}
