/**
 * [78] Subsets
 *
 * Given a set of distinct integers, nums, return all possible subsets (the power set).
 * 
 * Note: The solution set must not contain duplicate subsets.
 * 
 * Example:
 * 
 * 
 * Input: nums = [1,2,3]
 * Output:
 * [
 *   [3],
 *   [1],
 *   [2],
 *   [1,2,3],
 *   [1,3],
 *   [2,3],
 *   [1,2],
 *   []
 * ]
 * 
 */
/// leetcode rust bug
/// recursion with nums slice & result size
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
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
    fn test_78() {
        assert_vec2_eq(&Solution::subsets(vec![1,2,3]), &vec2![
  [3],
  [1],
  [2],
  [1,2,3],
  [1,3],
  [2,3],
  [1,2],
  []
]);
        assert_vec2_eq(&Solution::subsets(vec![]), &vec![vec![0;0];1]);
    }
}
