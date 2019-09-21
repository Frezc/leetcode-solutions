/**
 * [131] Palindrome Partitioning
 *
 * Given a string s, partition s such that every substring of the partition is a palindrome.
 * 
 * Return all possible palindrome partitioning of s.
 * 
 * Example:
 * 
 * 
 * Input: "aab"
 * Output:
 * [
 *   ["aa","b"],
 *   ["a","a","b"]
 * ]
 * 
 * 
 */
/// a DFS solution
/// 1. split string to a, b parts by index. a = s[0..i]. b = s[i..];
/// 2. If a is palindrome(compare to rev),
///     if i is last index, return [a];
///     else pass b string to step 1 recursively, and get all possible split that pretend with a.
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() > 0 {
            return partition_chars(&chars);
        }
        vec![]
    }
}

fn partition_chars(chars: &[char]) -> Vec<Vec<String>> {
    let mut re = vec![];
    for i in 1..=chars.len() {
        if is_palindrome(&chars[..i]) {
            let el0: String = chars[..i].iter().collect();
            if i >= chars.len() {
                re.push(vec![el0]);
            } else {
                let mut child_result = partition_chars(&chars[i..]);
                child_result.iter_mut().for_each(|list| {
                    list.insert(0, el0.clone());
                });
                re.extend(child_result);
            }
        }
    }
    re
}

fn is_palindrome(chars: &[char]) -> bool {
    let len = chars.len();
    for i in 0..len {
        if chars[i] != chars[len - i - 1] {
            return false;
        }
    }
    true
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_131() {
        assert_vec2_eq(&Solution::partition("aab".to_string()), &vec![
          strvec!["aa","b"],
  strvec!["a","a","b"]
        ]);

        assert_vec2_eq(&Solution::partition("".to_string()), &vec![

        ]);

        assert_vec2_eq(&Solution::partition("ab".to_string()), &vec![
            strvec!["a","b"],

        ]);
    }
}
