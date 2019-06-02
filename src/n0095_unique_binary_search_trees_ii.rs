/**
 * [95] Unique Binary Search Trees II
 *
 * Given an integer n, generate all structurally unique BST's (binary search trees) that store values 1 ... n.
 * 
 * Example:
 * 
 * 
 * Input: 3
 * Output:
 * [
 *   [1,null,3,2],
 *   [3,2,null,1],
 *   [3,1,null,null,2],
 *   [2,1,3],
 *   [1,null,2,null,3]
 * ]
 * Explanation:
 * The above output corresponds to the 5 unique BST's shown below:
 * 
 *    1         3     3      2      1
 *     \       /     /      / \      \
 *      3     2     1      1   3      2
 *     /     /       \                 \
 *    2     1         2                 3
 * 
 * 
 */
/// For special input 0, we should return empty vector
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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![];
        }
        generate_trees(1, n)
    }
}

fn generate_trees(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if start > end {
        return vec![None];
    }

    if start == end {
        return vec![Some(Rc::new(RefCell::new(TreeNode::new(start))))];
    }

    let mut result = vec![];

    for i in start..=end {
        let left_trees = generate_trees(start, i - 1);
        let right_trees = generate_trees(i+1,end);

        for lt in left_trees.iter() {
            for rt in right_trees.iter() {
                let mut root = TreeNode::new(i);
                root.left = lt.as_ref().map(|t| t.clone());
                root.right = rt.as_ref().map(|t| t.clone());
                result.push( Some(Rc::new(RefCell::new(root))));
            }
        }

    }
    result
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_95() {
        assert_vec_eq(&Solution::generate_trees(3), &vec![
            tree![1,null,3,2],
            tree![3,2,null,1],
            tree![3,1,null,null,2],
            tree![2,1,3],
            tree![1,null,2,null,3]
        ]);
        assert_vec_eq(&Solution::generate_trees(0), &vec![]);
    }
}
