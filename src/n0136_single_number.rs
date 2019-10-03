/**
 * [136] Single Number
 *
 * Given a non-empty array of integers, every element appears twice except for one. Find that single one.
 * 
 * Note:
 * 
 * Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?
 * 
 * Example 1:
 * 
 * 
 * Input: [2,2,1]
 * Output: 1
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [4,1,2,1,2]
 * Output: 4
 * 
 * 
 */
/// use xor
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        for i in 1..nums.len() {
            nums[0] ^= nums[i];
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
    fn test_136() {
        assert_eq!(Solution::single_number(vec![2,2,1]), 1);
        assert_eq!(Solution::single_number(vec![4,1,2,1,2]), 4);

    }
}
