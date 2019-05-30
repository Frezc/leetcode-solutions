/**
 * [90] Subsets II
 *
 * Given a collection of integers that might contain duplicates, nums, return all possible subsets (the power set).
 * 
 * Note: The solution set must not contain duplicate subsets.
 * 
 * Example:
 * 
 * 
 * Input: [1,2,2]
 * Output:
 * [
 *   [2],
 *   [1],
 *   [1,2,2],
 *   [2,2],
 *   [1,2],
 *   []
 * ]
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result = vec![];
        for i in 0..=nums.len() {
            result.extend(subsets_with_size(&nums, i))
        }
        result
    }
}

fn subsets_with_size(nums: &Vec<i32>, size: usize) -> Vec<Vec<i32>> {
    if size == 0 {
        return vec![vec![]];
    }
    let mut result = vec![];

    for i in 0..=nums.len()-size {
        if i > 0 && nums[i] == nums[i-1] {
            continue;
        }
        if size == 1 {
            result.push(vec![nums[i]]);
        } else {
            result.extend(
                subsets_with_size(
                    &nums[i+1..].to_vec(),
                    size-1
                ).into_iter().map(|mut subset|{
                    subset.push(nums[i]);
                    subset
                })
            )
        }
    }

    result
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_90() {
        assert_vec2_eq(&Solution::subsets_with_dup(vec![1,2,2]), &vec2![
  [2],
  [1],
  [1,2,2],
  [2,2],
  [1,2],
  []
]);
        assert_vec2_eq(&Solution::subsets_with_dup(vec![]), &vec![vec![0;0];1]);
    }
}
