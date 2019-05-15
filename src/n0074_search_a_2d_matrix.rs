/**
 * [74] Search a 2D Matrix
 *
 * Write an efficient algorithm that searches for a value in an m x n matrix. This matrix has the following properties:
 * 
 * 
 * 	Integers in each row are sorted from left to right.
 * 	The first integer of each row is greater than the last integer of the previous row.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input:
 * matrix = [
 *   [1,   3,  5,  7],
 *   [10, 11, 16, 20],
 *   [23, 30, 34, 50]
 * ]
 * target = 3
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input:
 * matrix = [
 *   [1,   3,  5,  7],
 *   [10, 11, 16, 20],
 *   [23, 30, 34, 50]
 * ]
 * target = 13
 * Output: false
 * 
 */
/// binary search
/// Be careful with empty matrix
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut lower = 0;
        let row = matrix.len();
        let column = matrix.get(0).map_or(0, |a| a.len());
        let mut upper = row * column;
        if row == 0 || column == 0 {
            return false;
        }
        loop {
            let i = (lower + upper) / 2;
            let cur = matrix[i / column][i % column];
            if target == cur {
                return true;
            }

            if lower + 1 >= upper {
                return false;
            }

            if target > cur {
                lower = i;
            } else {
                upper = i;
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
    fn test_74() {
        assert_eq!(Solution::search_matrix(vec2![
  [1,   3,  5,  7],
  [10, 11, 16, 20],
  [23, 30, 34, 50]
], 3), true);
        assert_eq!(Solution::search_matrix(vec2![
  [1,   3,  5,  7],
  [10, 11, 16, 20],
  [23, 30, 34, 50]
], 13), false);
        assert_eq!(Solution::search_matrix(vec![], 0), false);
        assert_eq!(Solution::search_matrix(vec2![[], []], 0), false);
    }
}
