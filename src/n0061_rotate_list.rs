/**
 * [61] Rotate List
 *
 * Given a linked list, rotate the list to the right by k places, where k is non-negative.
 * 
 * Example 1:
 * 
 * 
 * Input: 1->2->3->4->5->NULL, k = 2
 * Output: 4->5->1->2->3->NULL
 * Explanation:
 * rotate 1 steps to the right: 5->1->2->3->4->NULL
 * rotate 2 steps to the right: 4->5->1->2->3->NULL
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 0->1->2->NULL, k = 4
 * Output: 2->0->1->NULL
 * Explanation:
 * rotate 1 steps to the right: 2->0->1->NULL
 * rotate 2 steps to the right: 1->2->0->NULL
 * rotate 3 steps to the right: 0->1->2->NULL
 * rotate 4 steps to the right: 2->0->1->NULL
 * 
 */
/// A simple linked list problem, but not easy to rust
/// when calling `next` on a node, you will take it's immutable borrow
/// so you're not able to mutate it in same scope
/// Check this [article](https://rcoh.me/posts/rust-linked-list-basically-impossible/) for more information
/// and there is a [book](https://rust-unofficial.github.io/too-many-lists/) maybe helpful
///
/// In this solution, I just create new node to avoid borrow check.
///
/// More elegant solution is pointing tail's next to head to create a circular linked list,
/// then step `size - k % size`. You will get tail of result list,
/// and next node is the head of result list.
pub struct Solution {}
use super::linked_list::{ListNode,to_list};

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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut size = 0;
        let h = Box::new(ListNode {
            val: 0,
            next: head
        });
        let mut tail = &h;
        while let Some(n) = tail.next.as_ref() {
            size += 1;
            tail = n;
        }

        if size > 0 {
            let mut h2 = ListNode::new(0);
            let offset = size - k % size;
            let mut iter = &h;
            let mut iter2 = &mut h2;
            for _ in 0..offset {
                iter = iter.next.as_ref().unwrap();
                iter2.next = Some(Box::new(ListNode::new(iter.val)));
                iter2 = iter2.next.as_mut().unwrap();
            }
            iter2.next = None;
            let mut h1 = ListNode::new(0);
            iter2 = &mut h1;
            for _ in 0..size-offset {
                iter = iter.next.as_ref().unwrap();
                iter2.next = Some(Box::new(ListNode::new(iter.val)));
                iter2 = iter2.next.as_mut().unwrap();
            }
            iter2.next = h2.next;
            h1.next
        } else {
            h.next
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_61() {
        assert_eq!(Solution::rotate_right(linked![1,2,3,4,5], 2), linked![4,5,1,2,3]);
        assert_eq!(Solution::rotate_right(linked![1,2,3,4,5], 50002), linked![4,5,1,2,3]);
        assert_eq!(Solution::rotate_right(linked![0,1,2], 4), linked![2,0,1]);
        assert_eq!(Solution::rotate_right(linked![], 40), linked![]);
        assert_eq!(Solution::rotate_right(linked![1,2,3], 0), linked![1,2,3]);
        assert_eq!(Solution::rotate_right(linked![], 0), linked![]);
        assert_eq!(Solution::rotate_right(linked![1], 3), linked![1]);
    }
}
