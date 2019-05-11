/**
 * [62] Unique Paths
 *
 * A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
 * 
 * The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
 * 
 * How many possible unique paths are there?
 * 
 * <img src="https://assets.leetcode.com/uploads/2018/10/22/robot_maze.png" style="width: 400px; height: 183px;" /><br />
 * <small>Above is a 7 x 3 grid. How many possible unique paths are there?</small>
 * 
 * Note: m and n will be at most 100.
 * 
 * Example 1:
 * 
 * 
 * Input: m = 3, n = 2
 * Output: 3
 * Explanation:
 * From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
 * 1. Right -> Right -> Down
 * 2. Right -> Down -> Right
 * 3. Down -> Right -> Right
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: m = 7, n = 3
 * Output: 28
 * 
 */
/// A simple DP solution
/// Compute possible paths from 1*1 to m*n
/// For i(1~m)*j(1~n) board, if i or j equals 1 return 1, else return paths[i-1,j] + paths[i,j-1]
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut state = Vec::with_capacity(m);
        for i in 0..m {
            state.push(Vec::with_capacity(n));
            for j in 0..n {
                if i == 0 || j == 0 {
                    state[i].push(1);
                } else {
                    let new_value = state[i][j - 1] + state[i - 1][j];
                    state[i].push(new_value);
                }
            }
        }
        state[m - 1][n - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_62() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths(1, 1), 1);
        assert_eq!(Solution::unique_paths(2, 1), 1);
        assert_eq!(Solution::unique_paths(99, 1), 1);
        // 99,99 will overflow i32
//        assert_eq!(Solution::unique_paths(99, 99), 1);
    }
}
