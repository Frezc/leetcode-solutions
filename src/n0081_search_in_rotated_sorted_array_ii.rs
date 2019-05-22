/**
 * [81] Search in Rotated Sorted Array II
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
 * 
 * (i.e., [0,0,1,2,2,5,6] might become [2,5,6,0,0,1,2]).
 * 
 * You are given a target value to search. If found in the array return true, otherwise return false.
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [2,5,6,0,0,1,2], target = 0
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [2,5,6,0,0,1,2], target = 3
 * Output: false
 * 
 * Follow up:
 * 
 * 
 * 	This is a follow up problem to <a href="/problems/search-in-rotated-sorted-array/description/">Search in Rotated Sorted Array</a>, where nums may contain duplicates.
 * 	Would this affect the run-time complexity? How and why?
 * 
 * 
 */
/// The key point of this solution is checking mid in higher half or lower half.
/// If in lower half, like `4,5,0,1,3`, we can assume mid to end is monotonically increasing,
/// and just checking target is in range.
/// If in higher half, like `4,5,6,1,3`, we can assume start to mid is monotonically increasing,
/// and check if in range.
///
/// There is something difference to #33. Middle number may be same to first,
/// so we cannot determine target in lower or higher half. And we should check both.
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        search(&nums, target)
    }
}

fn search(nums: &[i32], target: i32) -> bool {
    if nums.len() == 0 {
        return false;
    }

    let mid = nums.len() / 2;
    if target == nums[0] || target == *nums.last().unwrap() || target == nums[mid] {
        return true;
    }

    if nums.len() <= 3 {
        return false;
    }

    if nums[mid] < nums[0] {
        // if mid element in lower half
        if target > nums[mid] && target < nums[0] {
            search(&nums[mid + 1..], target)
        } else {
            search(&nums[1..mid], target)
        }
    } else if nums[mid] > nums[0] {
        // if mid element in higher half
        if target > nums[0] && target < nums[mid] {
            search(&nums[1..mid], target)
        } else {
            search(&nums[mid + 1..], target)
        }
    } else {
        // we don't know if mid in lower or higher
        search(&nums[1..mid], target) || search(&nums[mid + 1..], target)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_81() {
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 0, 0, 0, 1, 2], 6), true);
        assert_eq!(Solution::search(vec![2, 2, 2, 0, 0, 0, 0, 0, 2, 2], 6), false);
        assert_eq!(Solution::search(vec![2, 2, 3, 0, 0, 0, 0, 0, 2, 2], 3), true);
        assert_eq!(Solution::search(vec![2, 2, 2, 2, 2, 2, 2, 2, 2], 3), false);
        assert_eq!(Solution::search(vec![], 3), false);
        assert_eq!(Solution::search(vec![1, 1, 1, 3, 1], 3), true);
        assert_eq!(Solution::search(vec![3, 1, 3, 3, 3], 1), true);
    }
}
