
/**
 * [47] Permutations II
 *
 * Given a collection of numbers that might contain duplicates, return all possible unique permutations.
 * 
 * Example:
 * 
 * 
 * Input: [1,1,2]
 * Output:
 * [
 *   [1,1,2],
 *   [1,2,1],
 *   [2,1,1]
 * ]
 * 
 * 
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashSet;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() > 1 {
            let mut uniq_set = HashSet::with_capacity(nums.len());
            nums.iter().filter(|&n| match uniq_set.get(n) {
                Some(_) => false,
                None => {
                    uniq_set.insert(n);
                    true
                }
            }).flat_map(|&n| {
                let mut filtered = false;
                Solution::permute_unique(
                    nums.iter()
                        .filter(|&&n1| if n1 == n && !filtered {
                            filtered = true;
                            false
                        } else { true })
                        .map(|&n| n)
                        .collect()
                ).into_iter().map(move |arr| [vec![n], arr].concat())
            }).collect()
        } else if nums.len() == 1 {
            vec![nums]
        } else {
            vec![]
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_47() {
        assert_eq!(Solution::permute_unique(vec![1, 1, 2]), vec![
            vec![1, 1, 2],
            vec![1, 2, 1],
            vec![2, 1, 1],
        ]);
        assert_eq!(Solution::permute_unique(vec![]), vec![vec![1; 0]; 0]);
        assert_eq!(Solution::permute_unique(vec![1]), vec![vec![1]]);
        assert_eq!(Solution::permute_unique(vec![1, 1]), vec![vec![1, 1]]);
    }
}
