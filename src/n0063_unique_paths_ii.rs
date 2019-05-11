/**
 * [63] Unique Paths II
 *
 * A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
 * 
 * The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
 * 
 * Now consider if some obstacles are added to the grids. How many unique paths would there be?
 * 
 * <img src="https://assets.leetcode.com/uploads/2018/10/22/robot_maze.png" style="width: 400px; height: 183px;" />
 * 
 * An obstacle and empty space is marked as 1 and 0 respectively in the grid.
 * 
 * Note: m and n will be at most 100.
 * 
 * Example 1:
 * 
 * 
 * Input:
 * [
 *   [0,0,0],
 *   [0,1,0],
 *   [0,0,0]
 * ]
 * Output: 2
 * Explanation:
 * There is one obstacle in the middle of the 3x3 grid above.
 * There are two ways to reach the bottom-right corner:
 * 1. Right -> Right -> Down -> Down
 * 2. Down -> Down -> Right -> Right
 * 
 * 
 */
/// Similar to #62
/// Add obstacles judging and set paths number to 0.
/// Pay attention to edge path, we should check previous path number now.
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid.first().map_or(0, |f| f.len());
        let mut state = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                if obstacle_grid[i][j] == 1 {
                    state[i][j] = 0;
                } else if i == 0 && j == 0 {
                    state[i][j] = 1;
                } else if i == 0 {
                    if state[i][j - 1] != 0 {
                        state[i][j] = 1;
                    } else {
                        state[i][j] = 0;
                    }
                } else if j == 0 {
                    if state[i-1][j] != 0 {
                        state[i][j] = 1;
                    } else {
                        state[i][j] = 0;
                    }
                } else {
                    let new_value = state[i][j - 1] + state[i - 1][j];
                    state[i][j] = new_value;
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
    fn test_63() {
        assert_eq!(Solution::unique_paths_with_obstacles(vec2![
          [0,0,0],
          [0,1,0],
          [0,0,0]
        ]), 2);
        assert_eq!(Solution::unique_paths_with_obstacles(vec2![
          [0,0,0],
          [1,1,0],
          [0,0,0]
        ]), 1);
        assert_eq!(Solution::unique_paths_with_obstacles(vec2![
          [0,1,0],
          [1,1,0],
          [0,0,0]
        ]), 0);
        assert_eq!(Solution::unique_paths_with_obstacles(vec2![
          [0,0],
          [1,1],
          [0,0]
        ]), 0);
        assert_eq!(Solution::unique_paths_with_obstacles(vec2![
          [0,0],
          [0,0],
          [0,0]
        ]), 3);
        assert_eq!(Solution::unique_paths_with_obstacles(vec2![
          [0,1],
          [1,0]
        ]), 0);
    }
}
