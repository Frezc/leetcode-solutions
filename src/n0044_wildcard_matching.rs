
/**
 * [44] Wildcard Matching
 *
 * Given an input string (s) and a pattern (p), implement wildcard pattern matching with support for '?' and '*'.
 * 
 * 
 * '?' Matches any single character.
 * '*' Matches any sequence of characters (including the empty sequence).
 * 
 * 
 * The matching should cover the entire input string (not partial).
 * 
 * Note:
 * 
 * 
 * 	s could be empty and contains only lowercase letters a-z.
 * 	p could be empty and contains only lowercase letters a-z, and characters like <font face="monospace">?</font> or *.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input:
 * s = "aa"
 * p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 * 
 * 
 * Example 2:
 * 
 * 
 * Input:
 * s = "aa"
 * p = "*"
 * Output: true
 * Explanation: '*' matches any sequence.
 * 
 * 
 * Example 3:
 * 
 * 
 * Input:
 * s = "cb"
 * p = "?a"
 * Output: false
 * Explanation: '?' matches 'c', but the second letter is 'a', which does not match 'b'.
 * 
 * 
 * Example 4:
 * 
 * 
 * Input:
 * s = "adceb"
 * p = "*a*b"
 * Output: true
 * Explanation: The first '*' matches the empty sequence, while the second '*' matches the substring "dce".
 * 
 * 
 * Example 5:
 * 
 * 
 * Input:
 * s = "acdcb"
 * p = "a*c?b"
 * Output: false
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut cache = Cache {
            cache: vec![vec![0; p.len() + 1]; s.len() + 1],
        };
        cache.match_part(&s.chars().collect::<Vec<char>>(), &p.chars().collect::<Vec<char>>(), 0, 0)
    }
}

// use cache to reduce duplicate check, bound O(m, n) to m * n
struct Cache {
    cache: Vec<Vec<i8>>,
}

impl Cache {
    // use index to avoid slice
    fn match_part(&mut self, s: &[char], p: &[char], i: usize, j: usize) -> bool {
        if self.cache[i][j] != 0 {
            return self.cache[i][j] == 1;
        }
        if i == s.len() && j == p.len() {
            self.cache[i][j] = 1;
            return true;
        }
        let result = if j < p.len() {
            match p[j] {
                '*' => if i < s.len() {
                    // if s exist, like 'aa', try '*' shallow char one by one
                    for i1 in i..s.len() + 1 {
                        if self.match_part(&s, &p, i1, j + 1) {
                            return true;
                        }
                    }
                    // if not matched, it means rest pattern cannot match, like 'aa' to '*b'
                    false
                } else {
                    // if str empty, pass rest pattern
                    self.match_part(&s, &p, i, j + 1)
                },
                '?' => if i < s.len() {
                    // match every char
                    self.match_part(&s, &p, i + 1, j + 1)
                } else {
                    false
                },
                c @ _ => if i < s.len() {
                    // match specific char
                    if s[i] == c {
                        self.match_part(&s, &p, i + 1, j + 1)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        } else {
            false
        };
        self.cache[i][j] = if result { 1 } else { -1 };
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_44() {
        assert!(!Solution::is_match("aa".to_string(), "a".to_string()));
        assert!(Solution::is_match("aa".to_string(), "*".to_string()));
        assert!(!Solution::is_match("cb".to_string(), "?a".to_string()));
        assert!(Solution::is_match("adceb".to_string(), "*a*b".to_string()));
        assert!(!Solution::is_match("acdcb".to_string(), "a*c?b".to_string()));
        assert!(!Solution::is_match("".to_string(), "*?".to_string()));
        assert!(Solution::is_match("".to_string(), "*".to_string()));
        assert!(Solution::is_match("aaaa".to_string(), "***a".to_string()));
    }

    #[test]
    fn test_44_failed() {
        assert!(!Solution::is_match("bbbbbbbabbaabbabbbbaaabbabbabaaabbababbbabbbabaaabaab".to_string(), "b*b*ab**ba*b**b***bba".to_string()));
    }

    #[test]
    fn test_44_failed1() {
        assert!(!Solution::is_match("baaaababbbaabbbbabbababaababaaabaababbbaabaabbbbaabbbbbbaabaabbababaaaaaaaabbbaaabbbababbbbbabbbbabbbbabbaabaababababbbababbbbbbbaaaaabbbbabbbbbaabbbaaaabaaabbabbabaabbbbabbbabbbaababbabbaaaababbababa".to_string(), "*bb*abb*a**ba**aba*b*bbb*abbaaa*bb*b**a*b*b**a**abb***ab*b**b*b*a******a*a*babaa*bab*a*b****bb*babb*baa".to_string()));
    }
}
