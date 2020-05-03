/**
 * [147] Insertion Sort List
 *
 * Sort a linked list using insertion sort.
 * 
 * <ol>
 * </ol>
 * 
 * <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0f/Insertion-sort-example-300px.gif" style="height:180px; width:300px" /><br />
 * <small>A graphical example of insertion sort. The partial sorted list (black) initially contains only the first element in the list.<br />
 * With each iteration one element (red) is removed from the input data and inserted in-place into the sorted list</small><br />
 *  
 * 
 * <ol>
 * </ol>
 * 
 * Algorithm of Insertion Sort:
 * 
 * <ol>
 * 	Insertion sort iterates, consuming one input element each repetition, and growing a sorted output list.
 * 	At each iteration, insertion sort removes one element from the input data, finds the location it belongs within the sorted list, and inserts it there.
 * 	It repeats until no input elements remain.
 * </ol>
 * 
 * <br />
 * Example 1:
 * 
 * 
 * Input: 4->2->1->3
 * Output: 1->2->3->4
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: -1->5->3->4->0
 * Output: -1->0->3->4->5
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
    /// 这题只要新建node就好，clone无敌
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = Box::new(ListNode::new(0));
        let mut iter = &head;
        while let Some(node) = iter {
            let mut iter2 = &mut result;
            let mut new_node = Box::new(ListNode::new(node.val));
            while let Some(next) = iter2.next.as_ref() {
                if new_node.val < next.val {
                    new_node.next = Some(next.clone());
                    break;
                }
                iter2 = iter2.next.as_mut().unwrap();
            }
            iter2.next = Some(new_node);
            iter = &node.next;
        }
        result.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_147() {
        assert_eq!(Solution::insertion_sort_list(linked![4,2,1,3]), linked![1,2,3,4]);
        assert_eq!(Solution::insertion_sort_list(linked![-1,5,3,4,0]), linked![-1,0,3,4,5]);
        assert_eq!(Solution::insertion_sort_list(linked![]), linked![]);
        assert_eq!(Solution::insertion_sort_list(linked![1]), linked![1]);
    }
}
