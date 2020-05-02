
/**
 * [143] Reorder List
 *
 * Given a singly linked list L: L0&rarr;L1&rarr;&hellip;&rarr;Ln-1&rarr;Ln,<br />
 * reorder it to: L0&rarr;Ln&rarr;L1&rarr;Ln-1&rarr;L2&rarr;Ln-2&rarr;&hellip;
 * 
 * You may not modify the values in the list's nodes, only nodes itself may be changed.
 * 
 * Example 1:
 * 
 * 
 * Given 1->2->3->4, reorder it to 1->4->2->3.
 * 
 * Example 2:
 * 
 * 
 * Given 1->2->3->4->5, reorder it to 1->5->2->4->3.
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
///
/// iterate with a reverse list
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut stack = vec![];
        let mut iter = head.as_ref();
        while let Some(node) = iter {
            stack.push(node.val);
            iter = node.next.as_ref();
        }

        let mut iter = head;
        let mut rev = false;
        let mut stack_iter = stack.iter();
        let mut stack_rev_iter = stack.iter().rev();
        while let Some(node) = iter.as_mut() {
            node.val = *if rev { stack_rev_iter.next().unwrap() } else { stack_iter.next().unwrap() };
            rev = !rev;
            iter = &mut node.next;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_143() {
        let mut l1 = linked![1,2,3,4];
        Solution::reorder_list(&mut l1);
        assert_eq!(l1, linked![1,4,2,3]);

        let mut l1 = linked![1,2,3,4,5];
        Solution::reorder_list(&mut l1);
        assert_eq!(l1, linked![1,5,2,4,3]);

        let mut l1 = linked![];
        Solution::reorder_list(&mut l1);
        assert_eq!(l1, linked![]);

        let mut l1 = linked![1];
        Solution::reorder_list(&mut l1);
        assert_eq!(l1, linked![1]);
    }
}
