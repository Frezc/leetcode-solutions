/**
 * [72] Edit Distance
 *
 * Given two words word1 and word2, find the minimum number of operations required to convert word1 to word2.
 * 
 * You have the following 3 operations permitted on a word:
 * 
 * <ol>
 * 	Insert a character
 * 	Delete a character
 * 	Replace a character
 * </ol>
 * 
 * Example 1:
 * 
 * 
 * Input: word1 = "horse", word2 = "ros"
 * Output: 3
 * Explanation: 
 * horse -> rorse (replace 'h' with 'r')
 * rorse -> rose (remove 'r')
 * rose -> ros (remove 'e')
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: word1 = "intention", word2 = "execution"
 * Output: 5
 * Explanation: 
 * intention -> inention (remove 't')
 * inention -> enention (replace 'i' with 'e')
 * enention -> exention (replace 'n' with 'x')
 * exention -> exection (replace 'n' with 'c')
 * exection -> execution (insert 'u')
 * 
 * 
 */
/// A DP solution.
/// First, let's see some edge case:
/// 1. word1 is "", word2 is "abc". Result is 3, we should insert 3 characters.
/// 2. word1 is "abc", word2 is "". Result is 3, we should delete 3 characters.
///
/// Second, declare two variable i & j, to indicate the substring word1[0..i] & word2[0..j].
/// Loop i & j to index every characters, and see logic below.
///
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        if word1.len() == 0 || word2.len() == 0 {
            return word1.len().max(word2.len()) as i32;
        }
        let mut dp = vec![vec![0; word2.len()+1]; word1.len()+1];
        for i in 0..=word1.len() {
            for j in 0..=word2.len() {
                if i == 0 {
                    // i:0 "" -> j:3 "abc", 3
                    dp[i][j] = j;
                } else if j == 0 {
                    // i:3 "abc" -> j:0 "", 3
                    dp[i][j] = i;
                } else if &word1[i-1..i] == &word2[j-1..j] {
                    // "bcd" -> "aabd", same as "bc" -> "aab"
                    dp[i][j] = dp[i-1][j-1];
                } else {
                    // "abc" -> "aab", if we insert 'b' to word1,
                    // destruct it like "abc" + "b" -> "aa" + "b".
                    // We could get distance equal "abc" -> "aa" + 1
                    let insert = dp[i][j-1] + 1;
                    // "abc" -> "aab", if we replace 'c' to 'b'
                    // get "ab" + "b" -> "aa" + "b"
                    let replace = dp[i-1][j-1] + 1;
                    // "abc" -> "aab", if we delete 'c'
                    // get "ab" -> "aab" + 1
                    let delete = dp[i-1][j] + 1;
                    // get min distance
                    dp[i][j] = insert.min(replace).min(delete);
                }
            }
        }
        dp[word1.len()][word2.len()] as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_72() {
        assert_eq!(Solution::min_distance("horse".to_string(), "ros".to_string()), 3);
        assert_eq!(Solution::min_distance("intention".to_string(), "execution".to_string()), 5);
        assert_eq!(Solution::min_distance("".to_string(), "".to_string()), 0);
        assert_eq!(Solution::min_distance("horse".to_string(), "".to_string()), 5);
    }
}
