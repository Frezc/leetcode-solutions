/**
 * [83] Remove Duplicates from Sorted List
 *
 * Given a sorted linked list, delete all duplicates such that each element appear only once.
 * 
 * Example 1:
 * 
 * 
 * Input: 1->1->2
 * Output: 1->2
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 1->1->2->3->3
 * Output: 1->2->3
 * 
 * 
 */
/// In rust, we cannot move field from reference. The quickest solution is clone 'next node'.
pub struct Solution {}
use super::linked_list::{ListNode, to_list};

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
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut iter = head.as_mut();
        while let Some(node) = iter {
            while let Some(nn) = node.next.as_ref() {
                if node.val == nn.val {
                    node.next = nn.next.clone();
                } else {
                    break;
                }
            }
            iter = node.next.as_mut();
        }
        head
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_83() {
        assert_eq!(Solution::delete_duplicates(to_list(vec![1,2,3,3,4,4,5])), to_list(vec![1,2,3,4,5]));
        assert_eq!(Solution::delete_duplicates(to_list(vec![1,1,1,2,3])), to_list(vec![1,2,3]));
        assert_eq!(Solution::delete_duplicates(to_list(vec![])), to_list(vec![]));
        assert_eq!(Solution::delete_duplicates(to_list(vec![1,1])), to_list(vec![1]));
    }
}
