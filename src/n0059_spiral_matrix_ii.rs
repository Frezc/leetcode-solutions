/**
 * [59] Spiral Matrix II
 *
 * Given a positive integer n, generate a square matrix filled with elements from 1 to n^2 in spiral order.
 * 
 * Example:
 * 
 * 
 * Input: 3
 * Output:
 * [
 *  [ 1, 2, 3 ],
 *  [ 8, 9, 4 ],
 *  [ 7, 6, 5 ]
 * ]
 * 
 * 
 */
/// Similar to #54, simulate the spiral path circle by circle.
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![1usize; n as usize]; n as usize];
        if n > 0 {
            fill_matrix(&mut result, 0, 1);
        }
        result.into_iter().map(|v| v.into_iter().map(|n|n as i32).collect()).collect()
    }
}

fn fill_matrix(res: &mut Vec<Vec<usize>>, circle: usize, start: usize) {
    let n = res.len();
    if circle < n / 2 {
        let line_size = n-1-2*circle;
        for i in 0..line_size {
            res[circle][circle + i] = start + i;
            res[circle + i][n - 1 - circle] = start + line_size + i;
            res[n - 1 - circle][n - 1 - circle - i] = start + 2 * line_size + i;
            res[n - 1 - circle - i][circle] = start + 3 * line_size + i;
        }
        fill_matrix(res, circle + 1, start + 4 * line_size);
    } else if circle * 2 + 1 == n {
        res[circle][circle] = start;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_59() {
        assert_eq!(Solution::generate_matrix(0), vec![vec![0i32; 0]; 0]);
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
        assert_eq!(Solution::generate_matrix(2), vec2![[1, 2], [4,3]]);
        assert_eq!(Solution::generate_matrix(3), vec2![
         [ 1, 2, 3 ],
         [ 8, 9, 4 ],
         [ 7, 6, 5 ]
        ]);
        assert_eq!(Solution::generate_matrix(4), vec2![[1,2,3,4],[12,13,14,5],[11,16,15,6],[10,9,8,7]]);
    }
}
