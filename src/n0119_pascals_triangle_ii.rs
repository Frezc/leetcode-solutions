/**
 * [119] Pascal's Triangle II
 *
 * Given a non-negative index k where k &le; 33, return the k^th index row of the Pascal's triangle.
 * 
 * Note that the row index starts from 0.
 * 
 * <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif" /><br />
 * <small>In Pascal's triangle, each number is the sum of the two numbers directly above it.</small>
 * 
 * Example:
 * 
 * 
 * Input: 3
 * Output: [1,3,3,1]
 * 
 * 
 * Follow up:
 * 
 * Could you optimize your algorithm to use only O(k) extra space?
 * 
 */
/// From https://leetcode.com/problems/pascals-triangle-ii/discuss/38420/Here-is-my-brief-O(k)-solution.
/// It's a very clever solution, and I'll describe it below.
/// First set first line to [1].
/// For second line, we'll start from [1, 0].
/// Calculate from back to front, plus front number to current:
/// [1,0] -> [1, 1].
///               ^ 0 + 1
/// [1,1,0] -> [1,1,1] -> [1，2，1]
///                 ^ 0 + 1   ^ 1 + 1
/// [1,2,1,0] -> [1,2,1,1] -> [1,2,3,1] -> [1,3,3,1]
///
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut result = vec![0i32; row_index as usize + 1];
        result[0] = 1;
        for i in 1..=row_index as usize {
            for j in (1..i+1).rev() {
                result[j] += result[j-1];
            }
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
    fn test_119() {
        assert_eq!(Solution::get_row(3), vec![1,3,3,1]);
    }
}
