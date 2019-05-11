/*
 * @lc app=leetcode id=64 lang=javascript
 *
 * [64] Minimum Path Sum
 *
 * https://leetcode.com/problems/minimum-path-sum/description/
 *
 * algorithms
 * Medium (46.50%)
 * Total Accepted:    226.5K
 * Total Submissions: 485.7K
 * Testcase Example:  '[[1,3,1],[1,5,1],[4,2,1]]'
 *
 * Given a m x n grid filled with non-negative numbers, find a path from top
 * left to bottom right which minimizes the sum of all numbers along its path.
 * 
 * Note: You can only move either down or right at any point in time.
 * 
 * Example:
 * 
 * 
 * Input:
 * [
 * [1,3,1],
 * ⁠ [1,5,1],
 * ⁠ [4,2,1]
 * ]
 * Output: 7
 * Explanation: Because the path 1→3→1→1→1 minimizes the sum.
 * 
 * 
 */
/**
 * @param {number[][]} grid
 * @return {number}
 */
var minPathSum = function(grid) {
  const x = grid.length;
  const y = grid[0] ? grid[0].length : 0;
  const state = new Array(x).fill([]);
  for(let i = 0; i < x; i++) {
    for (let j = 0; j < y; j++) {
      state[i][j] = grid[i][j] + (i > 0 || j > 0 ? Math.min(j-1 >= 0 ? state[i][j-1] : Infinity, i-1 >= 0 ? state[i-1][j] : Infinity) : 0);
    }
  }
  return state[x - 1][y - 1];
};

if ('it' in global) {
  it('min path sum', () => {
    expect(minPathSum([
      [1,3,1],
      [1,5,1],
      [4,2,1]
    ])).toBe(7);
    expect(minPathSum([
      [1,3],
      [1,5],
      [4,2]
    ])).toBe(8);
    expect(minPathSum([
      [1],
    ])).toBe(1);
  });
}
