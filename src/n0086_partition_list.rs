/**
 * [86] Partition List
 *
 * Given a linked list and a value x, partition it such that all nodes less than x come before nodes greater than or equal to x.
 * 
 * You should preserve the original relative order of the nodes in each of the two partitions.
 * 
 * Example:
 * 
 * 
 * Input: head = 1->4->3->2->5->2, x = 3
 * Output: 1->2->2->4->3->5
 * 
 * 
 */
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
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut less_head = ListNode::new(0);
        let mut less_iter = &mut less_head;
        let mut larger_head = ListNode::new(0);
        let mut larger_iter = &mut larger_head;
        while let Some(node) = head {
            if node.val < x {
                less_iter.next = Some(Box::new(ListNode::new(node.val)));
                less_iter = less_iter.next.as_mut().unwrap();
            } else {
                larger_iter.next = Some(Box::new(ListNode::new(node.val)));
                larger_iter = larger_iter.next.as_mut().unwrap();
            }

            head = node.next;
        }

        less_iter.next = larger_head.next;
        less_head.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_86() {
        assert_eq!(Solution::partition(to_list(vec![1,4,3,2,5,2]),3),to_list(vec![1,2,2,4,3,5]))
    }
}
