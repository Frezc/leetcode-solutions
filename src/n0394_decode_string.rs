
/**
 * [394] Decode String
 *
 * Given an encoded string, return its decoded string.
 * The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.
 * You may assume that the input string is always valid; No extra white spaces, square brackets are well-formed, etc.
 * Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. For example, there won't be input like 3a or 2[4].
 * Examples:
 * 
 * s = "3[a]2[bc]", return "aaabcbc".
 * s = "3[a2[c]]", return "accaccacc".
 * s = "2[abc]3[cd]ef", return "abcabccdcdcdef".
 * 
 *  
 * 
 */
pub struct Solution {}
use std::str::Chars;
use std::iter::Peekable;

// submission codes start here

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut res = String::new();
        let mut chs = s.chars().peekable();
        while chs.peek().is_some() {
            res.push_str(&decode_part(&mut chs));
        }
        res
    }
}

fn decode_part(chs: &mut Peekable<Chars>) -> String {
    let mut num = String::new();
    while let Some(&c) = chs.peek() {
        if c >= '0' && c <= '9' {
            num.push(c);
            chs.next();
        } else {
            break;
        }
    }
    if num.len() > 0 {
        chs.next();
    }
    let num = num.parse::<usize>().unwrap_or(1usize);
    let mut str_part = String::new();
    while let Some(&c) = chs.peek() {
        if c >= '0' && c <= '9' {
            str_part.push_str(&decode_part(chs));
        } else if c != ']' {
            str_part.push(c);
            chs.next();
        } else {
            chs.next();
            break;
        }
    }
    str_part.repeat(num)
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_394() {
        assert_eq!(Solution::decode_string("3[a]2[bc]".to_string()), "aaabcbc".to_string());
        assert_eq!(Solution::decode_string("3[a2[c]]".to_string()), "accaccacc".to_string());
        assert_eq!(Solution::decode_string("2[abc]3[cd]ef".to_string()), "abcabccdcdcdef".to_string());
        assert_eq!(Solution::decode_string("".to_string()), "".to_string());
        assert_eq!(Solution::decode_string("15[]".to_string()), "".to_string());
    }
}
