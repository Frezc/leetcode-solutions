/**
 * [45] Jump Game II
 *
 * Given an array of non-negative integers, you are initially positioned at the first index of the array.
 * 
 * Each element in the array represents your maximum jump length at that position.
 * 
 * Your goal is to reach the last index in the minimum number of jumps.
 * 
 * Example:
 * 
 * 
 * Input: [2,3,1,1,4]
 * Output: 2
 * Explanation: The minimum number of jumps to reach the last index is 2.
 *     Jump 1 step from index 0 to 1, then 3 steps to the last index.
 * 
 * Note:
 * 
 * You can assume that you can always reach the last index.
 * 
 */
pub struct Solution {}

// submission codes start here
/// We can split this question to some sub-question:
/// get the minimum number of jumps from an index to last index.
/// For example,
/// We jump the list from index 0: 3 2 1 1 1.
/// First element is 3, so we can jump 1,2,3 step.
/// For 1 step, the minimum jumps is `1 + jump([2,1,1,1])`,
/// and 2 => `1 + jump([1,1,1])`, 3 => `1 + jump([1,1])`.
/// Thus we get the minimum step is `1 + min(jump_from(1), jump_from(2), jump_from(3))`.
/// In order to prevent repetitive computation, we should add cache to every index.
///
/// 这道题还有很巧妙的bfs(https://leetcode.com/problems/jump-game-ii/discuss/18028/O(n)-BFS-solution), 左侧最优解和O(n)的贪心(https://leetcode.windliang.cc/leetCode-45-Jump-Game-II.html)
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut cache = Cache {
            cache: vec![-1; nums.len() + 1],
        };
        cache.jump_from(&nums, 0)
    }
}

struct Cache {
    cache: Vec<i32>,
}

impl Cache {
    // dp
    fn jump_from(&mut self, nums: &Vec<i32>, start: usize) -> i32 {
        if self.cache[start] != -1 {
            return self.cache[start];
        }
        if start >= nums.len() - 1 {
            self.cache[start] = 0;
            return 0;
        }
        if nums[start] == 0 {
            return -1;
        }
        let mut smallest = -1;
        let mut to = start + nums[start] as usize + 1;
        if to > nums.len() {
            to = nums.len();
        }
        for i in start + 1..to {
            // get the minimum step of rest jump
            let step = self.jump_from(nums, i);
            if step != -1 && (step < smallest || smallest == -1) {
                smallest = step;
            }
        }
        if smallest != -1 {
            self.cache[start] = smallest + 1;
            return self.cache[start];
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_45() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![2, 0, 1, 1, 4]), 3);
        assert_eq!(Solution::jump(vec![2]), 0);
        assert_eq!(Solution::jump(vec![1,9,2]), 2);
        assert_eq!(Solution::jump(vec![5,9,2]), 1);
    }
}
