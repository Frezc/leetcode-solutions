/**
 * [53] Maximum Subarray
 *
 * Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
 * 
 * Example:
 * 
 * 
 * Input: [-2,1,-3,4,-1,2,1,-5,4],
 * Output: 6
 * Explanation: [4,-1,2,1] has the largest sum = 6.
 * 
 * 
 * Follow up:
 * 
 * If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut previous = max;
        for i in 1..nums.len() {
            if previous > 0 {
                previous += nums[i];
            } else {
                previous = nums[i];
            }
            if previous > max {
                max = previous;
            }
        }
        max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_53() {
        assert_eq!(Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]),6);
        assert_eq!(Solution::max_sub_array(vec![-2,1,-3]),1);
        assert_eq!(Solution::max_sub_array(vec![-2]),-2);
        assert_eq!(Solution::max_sub_array(vec![-2,-3]),-2);
    }
}
