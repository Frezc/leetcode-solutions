/**
 * [109] Convert Sorted List to Binary Search Tree
 *
 * Given a singly linked list where elements are sorted in ascending order, convert it to a height balanced BST.
 * 
 * For this problem, a height-balanced binary tree is defined as a binary tree in which the depth of the two subtrees of every node never differ by more than 1.
 * 
 * Example:
 * 
 * 
 * Given the sorted linked list: [-10,-3,0,5,9],
 * 
 * One possible answer is: [0,-3,9,-10,null,5], which represents the following height balanced BST:
 * 
 *       0
 *      / \
 *    -3   9
 *    /   /
 *  -10  5
 * 
 * 
 */
pub struct Solution {}
use super::linked_list::{ListNode, to_list};
use super::tree::{TreeNode, to_tree};

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(nums[nums.len()/2])));
        let (left, right) = nums.split_at(nums.len()/2);
        root.borrow_mut().left = Solution::sorted_array_to_bst(left.to_vec());
        root.borrow_mut().right = Solution::sorted_array_to_bst(right[1..].to_vec());
        Some(root)
    }

    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::sorted_array_to_bst(head.map_or(vec![], |h| h.to_vec()))
    }
}

impl ListNode {
    pub fn to_vec(&self) -> Vec<i32> {
        let mut node = Some(self);
        let mut result = vec![];
        while let Some(n) = node {
            result.push(n.val);
            node = n.next.as_ref().map(|b| b.as_ref());
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_109() {
        assert_eq!(Solution::sorted_list_to_bst(to_list(vec![-10,-3,0,5,9])), tree![0,-3,9,-10,null,5]);

    }
}
