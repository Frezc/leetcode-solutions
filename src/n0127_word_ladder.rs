
/**
 * [127] Word Ladder
 *
 * Given two words (beginWord and endWord), and a dictionary's word list, find the length of shortest transformation sequence from beginWord to endWord, such that:
 * 
 * <ol>
 * 	Only one letter can be changed at a time.
 * 	Each transformed word must exist in the word list. Note that beginWord is not a transformed word.
 * </ol>
 * 
 * Note:
 * 
 * 
 * 	Return 0 if there is no such transformation sequence.
 * 	All words have the same length.
 * 	All words contain only lowercase alphabetic characters.
 * 	You may assume no duplicates in the word list.
 * 	You may assume beginWord and endWord are non-empty and are not the same.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input:
 * beginWord = "hit",
 * endWord = "cog",
 * wordList = ["hot","dot","dog","lot","log","cog"]
 * 
 * Output: 5
 * 
 * Explanation: As one shortest transformation is "hit" -> "hot" -> "dot" -> "dog" -> "cog",
 * return its length 5.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input:
 * beginWord = "hit"
 * endWord = "cog"
 * wordList = ["hot","dot","dog","lot","log"]
 * 
 * Output: 0
 * 
 * Explanation: The endWord "cog" is not in wordList, therefore no possible transformation.
 * 
 * 
 * 
 * 
 * 
 */
pub struct Solution {}

// submission codes start here
use std::collections::VecDeque;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
        if word_list.iter().find(|&s| s == &end_word).is_none() {
            return 0;
        }
        let mut queue = VecDeque::new();
        queue.push_back(begin_word);
        let mut level = 1;
        while !queue.is_empty() {
            let mut temp = vec![];
            while let Some(word) = queue.pop_front() {
                if word == end_word {
                    return level;
                }
                let mut rest = vec![];
                word_list.into_iter().for_each(|w| {
                    if is_transformed(&w, &word) {
                        temp.push(w);
                    } else {
                        rest.push(w);
                    }
                });
                word_list = rest;
            }
            level += 1;
            queue.extend(temp);
        }

        0
    }
}

fn is_transformed(s1: &str, s2: &str) -> bool {
    let mut count = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            count += 1;
        }

        if count > 1 {
            return false;
        }
    }

    return true;
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_127() {
        assert_eq!(Solution::ladder_length("hit".to_string(), "cog".to_string(),
                                           strvec!["hot","dot","dog","lot","log","cog"]), 5);
        assert_eq!(Solution::ladder_length("hit".to_string(), "cog".to_string(),
                                           strvec!["hot","dot","dog","lot","log"]), 0);
    }
}
