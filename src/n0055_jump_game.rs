/**
 * [55] Jump Game
 *
 * Given an array of non-negative integers, you are initially positioned at the first index of the array.
 * 
 * Each element in the array represents your maximum jump length at that position.
 * 
 * Determine if you are able to reach the last index.
 * 
 * Example 1:
 * 
 * 
 * Input: [2,3,1,1,4]
 * Output: true
 * Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [3,2,1,0,4]
 * Output: false
 * Explanation: You will always arrive at index 3 no matter what. Its maximum
 *              jump length is 0, which makes it impossible to reach the last index.
 * 
 * 
 */
/// Same as #45, but i use greedy solution here.
/// In the loop, we always compute the largest index we can reach from current index.
/// If current index large than the largest index, it means we are not able to the last index.
///
/// # Example
/// Input:           [2,3,1,1,4]
/// Reachable index: [0,2,4,3,4]
///                     ↑ return true here
///
/// Input:           [3,2,1,0,4]
/// Reachable index: [0,3,3,3, ]
///                         ↑ current index 4 > largest index(3), return false
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return true;
        }
        let mut max_index: usize = 0;
        let len = nums.len();
        for i in 0..len-1 {
            if i > max_index {
                return false;
            }
            let mi = i + nums[i] as usize;
            if mi > max_index {
                max_index = mi;
            }
            if max_index >= len - 1 {
                return true;
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_55() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump(vec![1, 0, 1, 1, 4]), false);
        assert_eq!(Solution::can_jump(vec![2]), true);
        assert_eq!(Solution::can_jump(vec![1,9,2]), true);
        assert_eq!(Solution::can_jump(vec![5,9,2]), true);
        assert_eq!(Solution::can_jump(vec![0, 2, 3]), false);
        assert_eq!(Solution::can_jump(vec![3,2,1,0,4]), false);
    }
}
