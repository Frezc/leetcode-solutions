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
/// A greedy solution.
/// 1. Loop from 1 to end, and mark each position to i. Storage first element to Sum.
/// 2. We can calculate the maximum sum of sub-array that end with i by follow step.
/// 3. If Sum > 0, add current number to Sum. Else assign number to Sum.
/// 4. Loop until i to end, we will get the largest sum.Return the max Sum.
///
/// # Example
/// Input: [-2,1,-3,4,-1,2,1,-5,4]
/// Sum:   [-2,1,-2,4, 3,5,6, 1,5]
/// Output: 6
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
