/**
 * [154] Find Minimum in Rotated Sorted Array II
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
 * 
 * (i.e.,  [0,1,2,4,5,6,7] might become  [4,5,6,7,0,1,2]).
 * 
 * Find the minimum element.
 * 
 * The array may contain duplicates.
 * 
 * Example 1:
 * 
 * 
 * Input: [1,3,5]
 * Output: 1
 * 
 * Example 2:
 * 
 * 
 * Input: [2,2,2,0,1]
 * Output: 0
 * 
 * Note:
 * 
 * 
 * 	This is a follow up problem to <a href="https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/description/">Find Minimum in Rotated Sorted Array</a>.
 * 	Would allow duplicates affect the run-time complexity? How and why?
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    /// find the first descend number or the first number
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut prev = nums[0];
        for &num in &nums[1..] {
            if num < prev {
                return num;
            }
            prev = num;
        }
        nums[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_154() {
        assert_eq!(Solution::find_min(vec![2,2,2,0,1]), 0);
        assert_eq!(Solution::find_min(vec![1,3,4]), 1);
        assert_eq!(Solution::find_min(vec![2]), 2);
        assert_eq!(Solution::find_min(vec![2,2,-2,0,1]), -2);
    }
}
