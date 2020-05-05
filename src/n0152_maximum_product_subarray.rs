/**
 * [152] Maximum Product Subarray
 *
 * Given an integer array nums, find the contiguous subarray within an array (containing at least one number) which has the largest product.
 * 
 * Example 1:
 * 
 * 
 * Input: [2,3,-2,4]
 * Output: 6
 * Explanation: [2,3] has the largest product 6.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [-2,0,-1]
 * Output: 0
 * Explanation: The result cannot be 2, because [-2,-1] is not a subarray.
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    /// 乘法相较加法的情况会少一点，我们先不考虑0的情况
    /// 如果数组里没有负数或者有偶数个负数，那么最大值肯定是所有数字相乘
    /// 所以只考虑奇数个负数的情况，这时候最大值肯定也是数组一头乘到另一头的第一个负数前（这样保证乘了偶数个负数）
    /// 综上所述，可以考虑正向和反向两次循环，在每次循环中乘上每一个数来得到最大值
    /// 另外考虑0的情况，0在这道题里其实相当于数组的分隔符了，可以考虑通过0分割数组，然后算每个数组的最大值
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut max = -2147483648;
        let mut temp = 1;
        for &num in nums.iter() {
            temp *= num;
            if temp > max {
                max = temp
            }
            if temp == 0 {
                temp = 1;
            }
        }

        temp = 1;
        for &num in nums.iter().rev() {
            temp *= num;
            if temp > max {
                max = temp
            }
            if temp == 0 {
                temp = 1;
            }
        }

        max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_152() {
        assert_eq!(Solution::max_product(vec![2,3,-2,4]), 6);
        assert_eq!(Solution::max_product(vec![-2,0,-1]), 0);
        assert_eq!(Solution::max_product(vec![-2,-1]), 2);
        assert_eq!(Solution::max_product(vec![-2]), -2);
        assert_eq!(Solution::max_product(vec![0,2]), 2);
        assert_eq!(Solution::max_product(vec![3,-1,4]), 4);
        assert_eq!(Solution::max_product(vec![]), 0);
    }
}
