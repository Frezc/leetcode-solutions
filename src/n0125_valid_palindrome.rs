/**
 * [125] Valid Palindrome
 *
 * Given a string, determine if it is a palindrome, considering only alphanumeric characters and ignoring cases.
 * 
 * Note: For the purpose of this problem, we define empty string as valid palindrome.
 * 
 * Example 1:
 * 
 * 
 * Input: "A man, a plan, a canal: Panama"
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: "race a car"
 * Output: false
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s.chars().filter(|c| c.is_ascii_alphanumeric()).map(|c| c.to_lowercase().to_string()).collect();
        for (a, b) in s.chars().zip(s.chars().rev()) {
            if a != b {
                return false;
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_125() {
        assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
        assert_eq!(Solution::is_palindrome("0P".to_string()), false);
        assert_eq!(Solution::is_palindrome("".to_string()), true);
    }
}
