/**
 * [91] Decode Ways
 *
 * A message containing letters from A-Z is being encoded to numbers using the following mapping:
 * 
 * 
 * 'A' -> 1
 * 'B' -> 2
 * ...
 * 'Z' -> 26
 * 
 * 
 * Given a non-empty string containing only digits, determine the total number of ways to decode it.
 * 
 * Example 1:
 * 
 * 
 * Input: "12"
 * Output: 2
 * Explanation: It could be decoded as "AB" (1 2) or "L" (12).
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: "226"
 * Output: 3
 * Explanation: It could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" ( 2 2 6).
 * 
 */
/// Solve it by fibonacci with dp.
/// If s[i-1,i] is in [0,26], set dp[i] = dp[i-1] + dp[i-2], else set dp[i] = dp[i-1]
/// But annoyed with "0" with its edge cases that are not mentioned in question.
/// Some cases:
/// 1. start with "0", e.g. "0101","0111". return 0.
/// 2. start without "0", set dp[0] = 1.
/// 3. standalone "0", e.g. "1001",          "301". return 0.
///                            ↑ back to "0"   ↑ cannot combine with previous
/// 4. if previous number is 0, we shouldn't combine it. e.g. "101", don't check "01", just set dp[i] = dp[i-1]
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut dp = vec![0; s.len()];
        for i in 0..s.len() {
            let cur = s[i..=i].parse::<u32>().unwrap();
            if cur == 0 {
                if i > 0 {
                    let last_num = s[i - 1..=i].parse::<u32>().unwrap();
                    if last_num <= 26 && last_num > 0 {
                        dp[i] = *dp.get((i as i32 - 2) as usize).unwrap_or(&1);
                        continue;
                    }
                }

                return 0;
            } else {
                if i > 0 {
                    dp[i] = dp[i - 1] + if &s[i-1..i] != "0" && s[i - 1..=i].parse::<u32>().unwrap() <= 26 {
                        *dp.get((i as i32 - 2) as usize).unwrap_or(&1)
                    } else { 0 }
                } else {
                    dp[i] = 1;
                }
            }
        }
        *dp.last().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use crate::*;

    use super::*;

    #[test]
    fn test_91() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
        assert_eq!(Solution::num_decodings("1".to_string()), 1);
        assert_eq!(Solution::num_decodings("22626".to_string()), 6);
        assert_eq!(Solution::num_decodings("0".to_string()), 0);
        assert_eq!(Solution::num_decodings("010".to_string()), 0);
        assert_eq!(Solution::num_decodings("1001".to_string()), 0);
        assert_eq!(Solution::num_decodings("1010".to_string()), 1);
        assert_eq!(Solution::num_decodings("101".to_string()), 1);
        assert_eq!(Solution::num_decodings("1110".to_string()), 2);
    }
}
