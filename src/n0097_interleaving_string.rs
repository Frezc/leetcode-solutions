/**
 * [97] Interleaving String
 *
 * Given s1, s2, s3, find whether s3 is formed by the interleaving of s1 and s2.
 * 
 * Example 1:
 * 
 * 
 * Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
 * Output: false
 * 
 * 
 */
/// This solution will be accepted even no dp.
/// Will dp make our solution faster ? Absolutely true. see below.
/// We can use start indices (i1, i2) here and suppose all elements is equal in two strings.
///                         (0, 0)
///                     (1, 0) (0, 1)
///                 (2, 0) (1, 1) (0, 2)
///             (3, 0) (2, 1) (1, 2) (0, 3)
///                    ↑↑↑↑↑↑↑↑↑↑↑↑↑ we can reuse the result of intersected nodes
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let mut c = Cache::new();

        let result = c.check_interleave(&s1, 0, &s2, 0, &s3);
        result
    }
}

use std::collections::HashMap;

struct Cache {
    cache: HashMap<String, bool>
}

impl Cache {
    fn new() -> Self {
        Cache {
            cache: HashMap::new()
        }
    }

    fn check_interleave(&mut self, s1: &str, mut i1: usize, s2: &str, mut i2: usize, s3: &str) -> bool {
        println!("i1:{}, i2: {}", i1, i2);
        let key = format!("{}-{}", i1, i2);
        if let Some(&re) = self.cache.get(&key) {
            return re;
        }
        let len = s3.len();

        let mut result = true;
        while i1 + i2 < len {
            let c1 = s1.get(i1..=i1);
            let c2 = s2.get(i2..=i2);
            let c3 = &s3[i1+i2..=i1+i2];
            if c1.is_none() {
                let c2 = c2.unwrap();
                if c2 == c3 {
                    i2 += 1;
                } else {
                    result = false;
                    break;
                }
            } else if c2.is_none() {
                let c1 = c1.unwrap();
                if c1 == c3 {
                    i1 += 1;
                } else {
                    result = false;
                    break;
                }
            } else {
                let c1 = c1.unwrap();
                let c2 = c2.unwrap();

                if c1 != c2 {
                    if c1 == c3 {
                        i1 += 1;
                    } else if c2 == c3 {
                        i2 += 1;
                    } else {
                        result = false;
                        break;
                    }
                } else if c1 == c3 {
                    if !self.check_interleave(s1, i1 + 1, s2, i2, s3)
                        && !self.check_interleave(s1, i1, s2, i2 + 1, s3) {
                        result = false;
                    }
                    break;
                } else {
                    result = false;
                    break;
                }
            }
        }

        self.cache.insert(key, result);
        return result;
    }
}



// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_97() {
        assert_eq!(Solution::is_interleave("aabcc".to_string(), "dbbca".to_string(), "aadbbcbcac".to_string()), true);
        assert_eq!(Solution::is_interleave("aacaac".to_string(), "aacaaeaac".to_string(), "aacaaeaaeaacaac".to_string()), false);
        assert_eq!(Solution::is_interleave((0..10).map(|_| "ab").collect(), (0..10).map(|_| "ac").collect(), (0..10).map(|_| "abac").collect()), false);
    }
}
