/**
 * [118] Pascal's Triangle
 *
 * Given a non-negative integer numRows, generate the first numRows of Pascal's triangle.
 * 
 * <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif" style="height:240px; width:260px" /><br />
 * <small>In Pascal's triangle, each number is the sum of the two numbers directly above it.</small>
 * 
 * Example:
 * 
 * 
 * Input: 5
 * Output:
 * [
 *      [1],
 *     [1,1],
 *    [1,2,1],
 *   [1,3,3,1],
 *  [1,4,6,4,1]
 * ]
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);
        for i in 1..=num_rows as usize {
            let mut level = Vec::with_capacity(i as usize);
            for j in 0..i {
                if j == 0 || j == i - 1 {
                    level.push(1);
                } else {
                    level.push(result[i - 2][j - 1] + result[i - 2][j]);
                }
            }
            result.push(level);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_118() {
        assert_eq!(Solution::generate(5), vec2![
     [1],
    [1,1],
   [1,2,1],
  [1,3,3,1],
 [1,4,6,4,1]
]);
    }
}
