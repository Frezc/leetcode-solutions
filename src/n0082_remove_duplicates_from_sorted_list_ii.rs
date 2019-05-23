/**
 * [82] Remove Duplicates from Sorted List II
 *
 * Given a sorted linked list, delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list.
 * 
 * Example 1:
 * 
 * 
 * Input: 1->2->3->3->4->4->5
 * Output: 1->2->5
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 1->1->1->2->3
 * Output: 2->3
 * 
 * 
 */
/// In rust, make use of as_ref and as_mut instead of '&' to reference Option<T>
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut h = ListNode {
            val: 0,
            next: None
        };
        let mut iter = &mut h;
        let mut iter2 = head.as_ref();
        let mut duplicate = false;
        while let Some(c) = iter2 {
            if iter.next.is_none() {
                iter.next = Some(Box::new(ListNode::new(c.val)));
                duplicate = false;
            } else if iter.next.as_ref().unwrap().val != c.val {
                if duplicate {
                    iter.next.as_mut().unwrap().val = c.val;
                } else {
                    iter = iter.next.as_mut().unwrap();
                    iter.next = Some(Box::new(ListNode::new(c.val)));
                }
                duplicate = false;
            } else {
                duplicate = true;
            }

            iter2 = c.next.as_ref();
        }

        if duplicate {
            iter.next = None;
        }
        h.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_82() {
        assert_eq!(Solution::delete_duplicates(to_list(vec![1,2,3,3,4,4,5])), to_list(vec![1,2,5]));
        assert_eq!(Solution::delete_duplicates(to_list(vec![1,1,1,2,3])), to_list(vec![2,3]));
        assert_eq!(Solution::delete_duplicates(to_list(vec![])), to_list(vec![]));
        assert_eq!(Solution::delete_duplicates(to_list(vec![1,1])), to_list(vec![]));
    }
}
