
/**
 * [49] Group Anagrams
 *
 * Given an array of strings, group anagrams together.
 * 
 * Example:
 * 
 * 
 * Input: ["eat", "tea", "tan", "ate", "nat", "bat"],
 * Output:
 * [
 *   ["ate","eat","tea"],
 *   ["nat","tan"],
 *   ["bat"]
 * ]
 * 
 * Note:
 * 
 * 
 * 	All inputs will be in lowercase.
 * 	The order of your output does not matter.
 * 
 * 
 */
/// use Hashmap
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::with_capacity(strs.len());
        for s in strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort();
            let key: String = chars.into_iter().collect();
            map.entry(key).or_insert(vec![]).push(s);
        }
        map.into_iter().map(|(_, v)| v).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_49() {
        assert_vec2_eq(
            &Solution::group_anagrams(strvec!["eat", "tea", "tan", "ate", "nat", "bat"]),
            &vec![
                strvec!["ate", "eat", "tea"],
                strvec!["nat", "tan"],
                strvec!["bat"]
            ],
        )
    }
}
