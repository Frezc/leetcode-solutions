/**
 * [87] Scramble String
 *
 * Given a string s1, we may represent it as a binary tree by partitioning it to two non-empty substrings recursively.
 * 
 * Below is one possible representation of s1 = "great":
 * 
 * 
 *     great
 *    /    \
 *   gr    eat
 *  / \    /  \
 * g   r  e   at
 *            / \
 *           a   t
 * 
 * 
 * To scramble the string, we may choose any non-leaf node and swap its two children.
 * 
 * For example, if we choose the node "gr" and swap its two children, it produces a scrambled string "rgeat".
 * 
 * 
 *     rgeat
 *    /    \
 *   rg    eat
 *  / \    /  \
 * r   g  e   at
 *            / \
 *           a   t
 * 
 * 
 * We say that "rgeat" is a scrambled string of "great".
 * 
 * Similarly, if we continue to swap the children of nodes "eat" and "at", it produces a scrambled string "rgtae".
 * 
 * 
 *     rgtae
 *    /    \
 *   rg    tae
 *  / \    /  \
 * r   g  ta  e
 *        / \
 *       t   a
 * 
 * 
 * We say that "rgtae" is a scrambled string of "great".
 * 
 * Given two strings s1 and s2 of the same length, determine if s2 is a scrambled string of s1.
 * 
 * Example 1:
 * 
 * 
 * Input: s1 = "great", s2 = "rgeat"
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: s1 = "abcde", s2 = "caebd"
 * Output: false
 * 
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        Cache {
            dp: HashMap::new()
        }.check_scramble(&s1, &s2)
    }
}

struct Cache {
    dp: HashMap<String, bool>
}

impl Cache {
    fn check_scramble(&mut self, s1: &str,  s2: &str) -> bool {
        let key = [s1, s2].join("|");
        if let Some(&re) = self.dp.get(&key) {
            return re;
        }
        if s1 == s2 {
            self.dp.insert(key, true);
            return true;
        }

        for i in 1..s1.len() {
            if self.check_scramble(&s1[0..i], &s2[0..i]) && self.check_scramble(&s1[i..], &s2[i..]) {
                self.dp.insert(key, true);
                return true;
            }
            if self.check_scramble(&s1[0..i], &s2[s2.len()-i..]) && self.check_scramble(&s1[i..], &s2[0..s2.len()-i]) {
                self.dp.insert(key, true);
                return true;
            }
        }

        self.dp.insert(key, false);
        false
    }
}


// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_87() {
        assert_eq!(Solution::is_scramble("great".to_string(), "rgeat".to_string()), true);
        assert_eq!(Solution::is_scramble("abcde".to_string(), "caebd".to_string()), false);
        assert_eq!(Solution::is_scramble("ccabcbabcbabbbbcbb".to_string(), "bbbbabccccbbbabcba".to_string()), false);
    }
}
