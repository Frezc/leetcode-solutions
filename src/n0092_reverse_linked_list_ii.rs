/**
 * [92] Reverse Linked List II
 *
 * Reverse a linked list from position m to n. Do it in one-pass.
 * 
 * Note: 1 &le; m &le; n &le; length of list.
 * 
 * Example:
 * 
 * 
 * Input: 1->2->3->4->5->NULL, m = 2, n = 4
 * Output: 1->4->3->2->5->NULL
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
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        let mut ori_iter = head.as_ref();
        let mut h = ListNode::new(0);
        let mut iter = &mut h;
        let mut i = 1;
        while let Some(node) = ori_iter {
            if i >= m {
                break;
            }
            iter.next = Some(Box::new(ListNode::new(node.val)));
            iter = iter.next.as_mut().unwrap();
            ori_iter = node.next.as_ref();
            i+=1;
        }

        let mut stack = vec![];
        while let Some(node) = ori_iter {
            if i > n {
                break;
            }
            stack.push(node.val);
            ori_iter = node.next.as_ref();
            i+=1;
        }


        for v in stack.into_iter().rev() {
            iter.next = Some(Box::new(ListNode::new(v)));
            iter = iter.next.as_mut().unwrap();
        }

        while let Some(node) = ori_iter {
            iter.next = Some(Box::new(ListNode::new(node.val)));
            iter = iter.next.as_mut().unwrap();
            ori_iter = node.next.as_ref();
        }

        h.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use crate::linked_list::arr_to_list;

    #[test]
    fn test_92() {
        assert_eq!(Solution::reverse_between(arr_to_list(&[1,2,3,4,5]), 2,4),arr_to_list(&[1,4,3,2,5]));
        assert_eq!(Solution::reverse_between(arr_to_list(&[1]), 1,1),arr_to_list(&[1]));
    }
}
