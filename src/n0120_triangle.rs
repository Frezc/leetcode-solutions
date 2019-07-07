/**
 * [120] Triangle
 *
 * Given a triangle, find the minimum path sum from top to bottom. Each step you may move to adjacent numbers on the row below.
 * 
 * For example, given the following triangle
 * 
 * 
 * [
 *      [2],
 *     [3,4],
 *    [6,5,7],
 *   [4,1,8,3]
 * ]
 * 
 * 
 * The minimum path sum from top to bottom is 11 (i.e., 2 + 3 + 5 + 1 = 11).
 * 
 * Note:
 * 
 * Bonus point if you are able to do this using only O(n) extra space, where n is the total number of rows in the triangle.
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.len() == 0 {
            return 0;
        }

        // O(n) extra space
        // This cache storage minimum sum of below lines
        let mut dp = vec![0i32; triangle.len()];

        for line in triangle.iter().rev() {
            // plus numbers of current line to cache
            dp.iter_mut()
                .zip(line)
                .for_each(|(a, b)| *a += *b);

            // get the smallest number for next line
            for i in 0..line.len() - 1 {
                dp[i] = dp[i].min(dp[i + 1]);
            }
        }

        dp[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_120() {
        assert_eq!(Solution::minimum_total(vec2![
     [2],
    [3,4],
   [6,5,7],
  [4,1,8,3]
]), 11);
        assert_eq!(Solution::minimum_total(vec2![[-1],[2,3],[1,-1,-3]]), -1);
    }
}
