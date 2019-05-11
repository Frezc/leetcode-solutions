/**
 * [67] Add Binary
 *
 * Given two binary strings, return their sum (also a binary string).
 * 
 * The input strings are both non-empty and contains only characters 1 or 0.
 * 
 * Example 1:
 * 
 * 
 * Input: a = "11", b = "1"
 * Output: "100"
 * 
 * Example 2:
 * 
 * 
 * Input: a = "1010", b = "1011"
 * Output: "10101"
 * 
 */
/// same with #66
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let max = b.len().max(a.len());
        let mut carry = false;
        let mut result = String::with_capacity(max);
        let mut iter1 = a.chars().into_iter().rev();
        let mut iter2 = b.chars().into_iter().rev();
        for _ in 0..max {
            let a = iter1.next().map_or(0, |n| n.to_digit(2).unwrap());
            let b = iter2.next().map_or(0, |n| n.to_digit(2).unwrap());
            let mut c = a + b;
            if carry {
                carry = false;
                c += 1;
            }
            if c / 2 > 0 {
                carry = true;
                c %= 2;
            }
            result.push_str(&c.to_string());
        }
        if carry {
            result.push('1');
        }
        result.chars().into_iter().rev().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_67() {
        assert_eq!(Solution::add_binary("11".to_string(), "1".to_string()), "100");
        assert_eq!(Solution::add_binary("1010".to_string(), "1011".to_string()), "10101");
        assert_eq!(Solution::add_binary("0".to_string(), "0".to_string()), "0");
    }
}
