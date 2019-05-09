/**
 * [52] N-Queens II
 *
 * The n-queens puzzle is the problem of placing n queens on an n&times;n chessboard such that no two queens attack each other.
 * 
 * <img src="https://assets.leetcode.com/uploads/2018/10/12/8-queens.png" style="width: 258px; height: 276px;" />
 * 
 * Given an integer n, return the number of distinct solutions to the n-queens puzzle.
 * 
 * Example:
 * 
 * 
 * Input: 4
 * Output: 2
 * Explanation: There are two distinct solutions to the 4-queens puzzle as shown below.
 * [
 *  [".Q..",  // Solution 1
 *   "...Q",
 *   "Q...",
 *   "..Q."],
 * 
 *  ["..Q.",  // Solution 2
 *   "Q...",
 *   "...Q",
 *   ".Q.."]
 * ]
 * 
 * 
 */
/// same as #51
pub struct Solution {}

// submission codes start here

impl Solution {
    // same logic with 51
    pub fn total_n_queens(n: i32) -> i32 {
        let mut pos_result = 0;
        let mut q_pos = Vec::with_capacity(n as usize);

        // reduce redundant loop, because results is symmetrical
        for i in 0..n / 2 {
            q_pos.push(i as usize);
            generate_n_queen(&mut pos_result, &mut q_pos, n as usize);
            q_pos.pop();
        }

        // reflect result
        pos_result *= 2;

        // special logic for mid column of odd size
        if n % 2 == 1 {
            q_pos.push(n as usize / 2);
            generate_n_queen(&mut pos_result, &mut q_pos, n as usize);
            q_pos.pop();
        }

        pos_result
    }
}

fn generate_n_queen(res: &mut i32, q_pos: &mut Vec<usize>, size: usize) {
    let r = q_pos.len();
    if r == size {
        *res += 1;
    }
    for i in 0..size {
        if is_valid(q_pos, i as usize) {
            q_pos.push(i);
            generate_n_queen(res, q_pos, size);
            q_pos.pop();
        }
    }
}

fn is_valid(q_pos: &Vec<usize>, col: usize) -> bool {
    let row = q_pos.len();
    for (r, c) in q_pos.iter().enumerate() {
        // do not use subtract here, usize does not allow minus number
        if r == row || *c == col || r + col == row + c || r + c == row + col {
            return false;
        }
    }
    true
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_52() {
        assert_eq!(Solution::total_n_queens(4), 2);
        assert_eq!(Solution::total_n_queens(1), 1);
        assert_eq!(Solution::total_n_queens(2), 0);
        assert_eq!(Solution::total_n_queens(3), 0);
//        assert_eq!(Solution::total_n_queens(300), 0);
    }
}
