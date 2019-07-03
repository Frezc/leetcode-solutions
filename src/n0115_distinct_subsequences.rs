/**
 * [115] Distinct Subsequences
 *
 * Given a string S and a string T, count the number of distinct subsequences of S which equals T.
 * 
 * A subsequence of a string is a new string which is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie, "ACE" is a subsequence of "ABCDE" while "AEC" is not).
 * 
 * Example 1:
 * 
 * 
 * Input: S = "rabbbit", T = "rabbit"
 * Output: 3
 * Explanation:
 * 
 * As shown below, there are 3 ways you can generate "rabbit" from S.
 * (The caret symbol ^ means the chosen letters)
 * 
 * rabbbit
 * ^^^^ ^^
 * rabbbit
 * ^^ ^^^^
 * rabbbit
 * ^^^ ^^^
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: S = "babgbag", T = "bag"
 * Output: 5
 * Explanation:
 * 
 * As shown below, there are 5 ways you can generate "bag" from S.
 * (The caret symbol ^ means the chosen letters)
 * 
 * babgbag
 * ^^ ^
 * babgbag
 * ^^    ^
 * babgbag
 * ^    ^^
 * babgbag
 *   ^  ^^
 * babgbag
 *     ^^^
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let l1 = t.len();
        let l2 = s.len();
        if l1 == 0 || l2 == 0 {
            return 0;
        }

        let mut dp = vec![vec![0; l2]; l1];


        for (i, c1) in t.chars().enumerate() {
            for (j, c2) in s.chars().enumerate() {
                if j < i || j > l2 - l1 + i {
                    continue;
                }

                if c1 == c2 {
                    dp[i][j] = if j > 0 {
                        dp[i][j - 1] + if i > 0 {
                            dp[i - 1][j - 1]
                        } else { 1 }
                    } else { 1 };
                } else {
                    dp[i][j] = if j > 0 {
                        dp[i][j -1]
                    } else { 0 }
                }
            }
        }
        dp[l1 - 1][l2 - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_115() {
        assert_eq!(Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()), 3);
        assert_eq!(Solution::num_distinct("babgbag".to_string(), "bag".to_string()), 5);
        assert_eq!(Solution::num_distinct("".to_string(), "bag".to_string()), 0);
        assert_eq!(Solution::num_distinct("".to_string(), "".to_string()), 0);
        assert_eq!(Solution::num_distinct("a".to_string(), "".to_string()), 0);
    }
}
