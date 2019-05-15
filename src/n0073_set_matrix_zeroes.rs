/**
 * [73] Set Matrix Zeroes
 *
 * Given a m x n matrix, if an element is 0, set its entire row and column to 0. Do it <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a>.
 * 
 * Example 1:
 * 
 * 
 * Input: 
 * [
 *   [1,1,1],
 *   [1,0,1],
 *   [1,1,1]
 * ]
 * Output: 
 * [
 *   [1,0,1],
 *   [0,0,0],
 *   [1,0,1]
 * ]
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 
 * [
 *   [0,1,2,0],
 *   [3,4,5,2],
 *   [1,3,1,5]
 * ]
 * Output: 
 * [
 *   [0,0,0,0],
 *   [0,4,5,0],
 *   [0,3,1,0]
 * ]
 * 
 * 
 * Follow up:
 * 
 * 
 * 	A straight forward solution using O(mn) space is probably a bad idea.
 * 	A simple improvement uses O(m + n) space, but still not the best solution.
 * 	Could you devise a constant space solution?
 * 
 * 
 */
/// Solution 1: space O(m*n)
/// save all position will setting to 0
///
/// Solution 2: O(m+n)
/// save all rows & columns will setting to 0
///
/// Solution 3: O(1)
/// 1. From (0,0) to (m-1, n-1), set first element of row & column to 0 when meet 0.
/// 2. From (m-1,n-1) to (0,0), set element to 0 when row or column leading by 0.
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut col0 = false;
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    if j == 0 {
                        col0 = true;
                    } else {
                        matrix[0][j] = 0;
                    }
                }
            }
        }

        for i in (0..matrix.len()).rev() {
            for j in (0..matrix[i].len()).rev() {
                if matrix[i][0] == 0 || j != 0 && matrix[0][j] == 0 || j == 0 && col0 {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_73() {
        let mut input1 = vec2![
          [1,1,1],
          [1,0,1],
          [1,1,1]
        ];
        Solution::set_zeroes(&mut input1);
        assert_eq!(input1, vec2![
            [1,0,1],
          [0,0,0],
          [1,0,1]
        ]);
        input1 = vec2![
  [0,1,2,0],
  [3,4,5,2],
  [1,3,1,5]
];
        Solution::set_zeroes(&mut input1);
        assert_eq!(input1, vec2![
  [0,0,0,0],
  [0,4,5,0],
  [0,3,1,0]
]);
        input1 = vec2![[-4,-2147483648,6,-7,0],[-8,6,-8,-6,0],[2147483647,2,-9,-6,-10]];
        Solution::set_zeroes(&mut input1);
        assert_eq!(input1, vec2![[0,0,0,0,0],[0,0,0,0,0],[2147483647,2,-9,-6,0]]);
    }
}
