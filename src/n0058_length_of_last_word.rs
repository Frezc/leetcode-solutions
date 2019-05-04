/**
 * [58] Length of Last Word
 *
 * Given a string s consists of upper/lower-case alphabets and empty space characters ' ', return the length of last word in the string.
 * 
 * If the last word does not exist, return 0.
 * 
 * Note: A word is defined as a character sequence consists of non-space characters only.
 * 
 * Example:
 * 
 * Input: "Hello World"
 * Output: 5
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end().split(' ').last().map_or(0, |w| w.len() as i32)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_58() {
        assert_eq!(Solution::length_of_last_word("Hello world".to_string()), 5);
        assert_eq!(Solution::length_of_last_word("Hello world  ".to_string()), 5);
        assert_eq!(Solution::length_of_last_word("Hello worl d  ".to_string()), 1);
    }
}
