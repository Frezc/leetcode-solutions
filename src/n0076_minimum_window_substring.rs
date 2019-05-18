
/**
 * [76] Minimum Window Substring
 *
 * Given a string S and a string T, find the minimum window in S which will contain all the characters in T in complexity O(n).
 * 
 * Example:
 * 
 * 
 * Input: S = "ADOBECODEBANC", T = "ABC"
 * Output: "BANC"
 * 
 * 
 * Note:
 * 
 * 
 * 	If there is no such window in S that covers all characters in T, return the empty string "".
 * 	If there is such window, you are guaranteed that there will always be only one unique minimum window in S.
 * 
 * 
 */
/// Sliding window solution.
/// 1. initial a window from 0 to j, which contains all chars in t.
/// 2. save the minimum length & move left side to next, until it break rule of step 1.
/// 3. move right side to next, until it contains all chars again or reach the end of string.
/// 4. loop 2 & 3, until j reach the end of string.
///
/// t may include repeat characters, so we should save it's count.
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut t_map: HashMap<char, usize> = HashMap::with_capacity(t.len());
        t.chars().for_each(|c| {
            *t_map.entry(c).or_insert(0) += 1;
        });

        let size = t.len();
        let mut cur_size = 0;
        let mut char_map = HashMap::with_capacity(t.len());

        let chars_vec: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = 0;
        let mut result = (false, 0, 0);
        while i <= j && j <= chars_vec.len() {
            if cur_size == size {
                // step 2
                if !result.0 || j - i < result.2 - result.1 {
                    result.1 = i;
                    result.2 = j;
                }
                result.0 = true;
                if t_map.contains_key(&chars_vec[i]) {
                    *char_map.entry(chars_vec[i]).or_insert(0) -= 1;
                    if char_map[&chars_vec[i]] < t_map[&chars_vec[i]] {
                        cur_size -= 1;
                    }
                }

                i += 1;
            } else {
                // step 1, 3
                if j == chars_vec.len() {
                    // edge condition, if j point to position after last
                    // we should only do step 2 instead of 3
                    break;
                }

                if t_map.contains_key(&chars_vec[j]) {
                    *char_map.entry(chars_vec[j]).or_insert(0) += 1;
                    if char_map[&chars_vec[j]] <= t_map[&chars_vec[j]] {
                        cur_size += 1;
                    }
                }

                j += 1;
            }
        }

        if result.0 {
            String::from(&s[result.1..result.2])
        } else {
            "".to_string()
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_76() {
        assert_eq!(Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC");
        assert_eq!(Solution::min_window("ABDWWE".to_string(), "ABC".to_string()), "");
        assert_eq!(Solution::min_window("BANC".to_string(), "".to_string()), "");
        assert_eq!(Solution::min_window("".to_string(), "A".to_string()), "");
        assert_eq!(Solution::min_window("a".to_string(), "aa".to_string()), "");
    }
}
