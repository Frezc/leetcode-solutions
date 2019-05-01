
/**
 * [1] Two Sum
 *
 * Given an array of integers, return indices of the two numbers such that they add up to a specific target.
 * 
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 * 
 * Example:
 * 
 * 
 * Given nums = [2, 7, 11, 15], target = 9,
 * 
 * Because nums[0] + nums[1] = 2 + 7 = 9,
 * return [0, 1].
 * 
 * 
 *  
 * 
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let map: HashMap<_, _> = nums.iter().enumerate().map(|(i,n)|(n,i)).collect();
        let (i1, n1) = nums.iter().enumerate()
            .find(|(i1, &n)| map.get(&(target - n)).map_or(false, |i2| i1 != i2)).unwrap();
        vec![i1 as i32, *map.get(&(target - n1)).unwrap() as i32]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11,15], 9), vec![0,1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1,2]);
    }
}
