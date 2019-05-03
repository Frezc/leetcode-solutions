/**
 * [51] N-Queens
 *
 * The n-queens puzzle is the problem of placing n queens on an n&times;n chessboard such that no two queens attack each other.
 * 
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/10/12/8-queens.png" style="width: 258px; height: 276px;" />
 * 
 * Given an integer n, return all distinct solutions to the n-queens puzzle.
 * 
 * Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space respectively.
 * 
 * Example:
 * 
 * 
 * Input: 4
 * Output: [
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
 * Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above.
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // iterate is enough
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut pos_result: Vec<Vec<usize>> = vec![];
        let mut q_pos = Vec::with_capacity(n as usize);

        // reduce redundant loop, because results is symmetrical
        for i in 0..n / 2 {
            q_pos.push(i as usize);
            generate_n_queen(&mut pos_result, &mut q_pos, n as usize);
            q_pos.pop();
        }

        // reflect result
        for i in 0..pos_result.len() {
            pos_result.push(pos_result[i].iter().map(|p| n as usize - 1 - p).collect());
        }

        // special logic for mid column of odd size
        if n % 2 == 1 {
            q_pos.push(n as usize / 2);
            generate_n_queen(&mut pos_result, &mut q_pos, n as usize);
            q_pos.pop();
        }

        // generate result
        pos_result.into_iter().map(|board| board.into_iter().map(|pos| {
            let mut char_vec = vec!['.'; n as usize];
            char_vec[pos] = 'Q';
            char_vec.iter().collect()
        }).collect()).collect()
    }
}

fn generate_n_queen(res: &mut Vec<Vec<usize>>, q_pos: &mut Vec<usize>, size: usize) {
    let r = q_pos.len();
    if r == size {
        res.push(q_pos.clone());
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
    fn test_51() {
        assert_vec2_eq(&Solution::solve_n_queens(4), &vec![
            strvec![".Q..",  // Solution 1
                 "...Q",
                 "Q...",
                 "..Q."],
            strvec!["..Q.",  // Solution 2
                 "Q...",
                 "...Q",
                 ".Q.."]
        ]);

        assert_vec2_eq(&Solution::solve_n_queens(1), &vec![
            strvec!["Q"],
        ]);

        assert_vec2_eq(&Solution::solve_n_queens(2), &vec![]);
        assert_vec2_eq(&Solution::solve_n_queens(3), &vec![]);
    }
}
