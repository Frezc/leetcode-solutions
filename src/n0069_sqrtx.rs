/**
 * [69] Sqrt(x)
 *
 * Implement int sqrt(int x).
 * 
 * Compute and return the square root of x, where x is guaranteed to be a non-negative integer.
 * 
 * Since the return type is an integer, the decimal digits are truncated and only the integer part of the result is returned.
 * 
 * Example 1:
 * 
 * 
 * Input: 4
 * Output: 2
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 8
 * Output: 2
 * Explanation: The square root of 8 is 2.82842..., and since 
 *              the decimal part is truncated, 2 is returned.
 * 
 * 
 */
/// Binary search solution
/// The upper bound should be limited to sqrt(2147483647) = 46340,
/// so as not to overflow integer.
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as u32;
        let mut result: u32 = 0;
        let mut lower: u32 = 0;
        let mut upper: u32 = x.min(46340) as u32;
        loop {
            if result * result > x {
                upper = result;
                result = (lower + result + 1) / 2;
            } else if (result + 1) * (result + 1) <= x {
                lower = result;
                result = (upper + result + 1) / 2;
            } else {
                break;
            }
        }
        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_69() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(1), 1);
        assert_eq!(Solution::my_sqrt(0), 0);
        assert_eq!(Solution::my_sqrt(100), 10);
        assert_eq!(Solution::my_sqrt(2147483647), 46340);
    }
}
