/**
 * [50] Pow(x, n)
 *
 * Implement <a href="http://www.cplusplus.com/reference/valarray/pow/" target="_blank">pow(x, n)</a>, which calculates x raised to the power n (x^<span style="font-size:10.8333px">n</span>).
 * 
 * Example 1:
 * 
 * 
 * Input: 2.00000, 10
 * Output: 1024.00000
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 2.10000, 3
 * Output: 9.26100
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: 2.00000, -2
 * Output: 0.25000
 * Explanation: 2^-2 = 1/2^2 = 1/4 = 0.25
 * 
 * 
 * Note:
 * 
 * 
 * 	-100.0 < x < 100.0
 * 	n is a 32-bit signed integer, within the range [-2^31, 2^31 - 1]
 * 
 * 
 */
/// Two key points:
/// 1. Solving this problem by dividing instead of loop, the latter will make time exceed.
/// # Example
/// Divide `my_pow(8.9, 99)` to `my_pow(8.9, 49) * my_pow(8.9, 50)`
/// 2. Keep 5 decimal place.
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        (my_pow_calc(x, n) * 100000f64).round() / 100000f64
    }
}

fn my_pow_calc(x: f64, n: i32) -> f64 {
    if n == 0 { 1f64 }
    else if n == 1 { x }
    else if n == -1 { 1f64 / x }
    else {
        let reverse = n < 0;
        let mut base = if !reverse {x} else {1f64/x};
        let mut multiple = 1;
        let abs_n = n.abs();
        while multiple <= abs_n / 2 {
            base *= base;
            multiple *= 2;
        }

        base * my_pow_calc(x, if reverse { n + multiple } else { n - multiple })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_50() {
        assert_eq!(Solution::my_pow(2f64, 10), 1024f64);
        assert_eq!(Solution::my_pow(2.1f64, 3), 9.26100f64);
        assert_eq!(Solution::my_pow(2f64, -2), 0.25f64);
        assert_eq!(Solution::my_pow(2f64, -1), 0.5f64);
        assert_eq!(Solution::my_pow(2f64, 0), 1f64);
        assert_eq!(Solution::my_pow(0f64, 0), 1f64);
        assert_eq!(Solution::my_pow(34.00515f64, -3), 3e-05f64);
        assert_eq!(Solution::my_pow(8.88023f64, 3), 700.28148f64);
    }

    #[test]
    fn test_50_large() {
        assert_eq!(Solution::my_pow(0.00001f64, 2147483647), 0f64);
    }
}
