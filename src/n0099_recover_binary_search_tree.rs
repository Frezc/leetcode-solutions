/**
 * [99] Recover Binary Search Tree
 *
 * Two elements of a binary search tree (BST) are swapped by mistake.
 * 
 * Recover the tree without changing its structure.
 * 
 * Example 1:
 * 
 * 
 * Input: [1,3,null,null,2]
 * 
 *    1
 *   /
 *  3
 *   \
 *    2
 * 
 * Output: [3,1,null,null,2]
 * 
 *    3
 *   /
 *  1
 *   \
 *    2
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [3,1,4,null,null,2]
 * 
 *   3
 *  / \
 * 1   4
 *    /
 *   2
 * 
 * Output: [2,1,4,null,null,3]
 * 
 *   2
 *  / \
 * 1   4
 *    /
 *   3
 * 
 * 
 * Follow up:
 * 
 * 
 * 	A solution using O(n) space is pretty straight forward.
 * 	Could you devise a constant space solution?
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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut node1: Option<Rc<RefCell<TreeNode>>> = None;
        let mut node2: Option<Rc<RefCell<TreeNode>>> = None;
        let mut pre: Option<Rc<RefCell<TreeNode>>> = None;
        let mut cur = root.as_ref().map(|node| node.clone());

        let mut compare_and_add = |node: &Rc<RefCell<TreeNode>>| {
            if let Some(pre) = &pre {
                if pre.borrow().val > node.borrow().val {
                    if node1.is_none() {
                        node1 = Some(pre.clone());
                        node2 = Some(node.clone());
                    } else {
                        node2 = Some(node.clone());
                    }
                }
            }
            pre = Some(node.clone());
        };

        while let Some(node) = cur {
            if let Some(left) = node.borrow().left.as_ref() {
                let predecessor = find_last(&node);

                if predecessor.borrow().right.is_some() {
                    predecessor.borrow_mut().right = None;
                    compare_and_add(&node);
                    cur = node.borrow().right.clone();
                } else {
                    predecessor.borrow_mut().right = Some(node.clone());
                    cur = Some(left.clone());
                }
            } else {
                compare_and_add(&node);
                cur = node.borrow().right.clone();
            }
        }

        let temp = node1.as_ref().unwrap().borrow().val;
        node1.unwrap().borrow_mut().val = node2.as_ref().unwrap().borrow_mut().val;
        node2.unwrap().borrow_mut().val = temp;
    }
}

fn find_last(root: &Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
    let mut result = root.borrow().left.as_ref().unwrap().clone();
    loop {
        if result.borrow().right.is_none() {
            break;
        };
        let node = result.clone();
        let node = node.borrow();
        let node = node.right.as_ref().unwrap();
        if node.borrow().val == root.borrow().val {
            break;
        }
        result = node.clone();
    }
    result
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_99() {
        let mut tree = tree![1,3,null,null,2];
        Solution::recover_tree(&mut tree);
        assert_eq!(tree, tree![3,1,null,null,2]);

        tree = tree![3,1,4,null,null,2];
        Solution::recover_tree(&mut tree);
        assert_eq!(tree, tree![2,1,4,null,null,3]);
    }
}
