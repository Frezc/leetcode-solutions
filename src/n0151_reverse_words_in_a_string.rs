/**
 * [151] Reverse Words in a String
 *
 * Given an input string, reverse the string word by word.
 * 
 *  
 * 
 * Example 1:
 * 
 * 
 * Input: "the sky is blue"
 * Output: "blue is sky the"
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: "  hello world!  "
 * Output: "world! hello"
 * Explanation: Your reversed string should not contain leading or trailing spaces.
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: "a good   example"
 * Output: "example good a"
 * Explanation: You need to reduce multiple spaces between two words to a single space in the reversed string.
 * 
 * 
 *  
 * 
 * Note:
 * 
 * 
 * 	A word is defined as a sequence of non-space characters.
 * 	Input string may contain leading or trailing spaces. However, your reversed string should not contain leading or trailing spaces.
 * 	You need to reduce multiple spaces between two words to a single space in the reversed string.
 * 
 * 
 *  
 * 
 * Follow up:
 * 
 * For C programmers, try to solve it in-place in O(1) extra space.
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    /// 这里用了很简单的需要额外空间的解法
    /// 关于in-place的解法，可以将整个字符串翻转然后再逐个单词翻转，别忘了去掉多余的空格
    pub fn reverse_words(s: String) -> String {
        s.split(" ").filter(|s| s.len() > 0).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join(" ")
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_151() {
        assert_eq!(Solution::reverse_words(String::from("  hello world!  ")), "world! hello".to_string());
        assert_eq!(Solution::reverse_words(String::from("a good   example")), "example good a".to_string());
        assert_eq!(Solution::reverse_words(String::from("    ")), "".to_string());
    }
}
