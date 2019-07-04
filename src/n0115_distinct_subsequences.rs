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
/// A simple DP question
/// Iterate solution:
/// Suppose i, j are end indices of substring of t and s, like s.substring(0, j+1) & t.substring(0, i+1)
/// Allocate dp with t.len * s.len, and fill 0 to it.
/// The key recursion is if s[j] == t[i], dp[i][j] = dp[i-1][j-1] + dp[i][j-1] else dp[i][j] = dp[i][j-1].
///
/// For example, suppose s = "babgbag", t = "bag", j = 3, i = 2.
/// Because s[j] == t[i], we can convert num_distinct("babg", "bag") to num_distinct("bab", "ba") + num_distinct("bab", "bag").
/// The first is matching current char and the second is not matching current char (just drop it).
///
/// Recursive solution:
/// Same
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
        assert_eq!(Solution::num_distinct("babgbag".to_string(), "bag".to_string()), 5);
        assert_eq!(Solution::num_distinct("bababa".to_string(), "bag".to_string()), 0);
        assert_eq!(Solution::num_distinct("".to_string(), "".to_string()), 0);
        assert_eq!(Solution::num_distinct("a".to_string(), "".to_string()), 0);
    }
}
