/**
 * [85] Maximal Rectangle
 *
 * Given a 2D binary matrix filled with 0's and 1's, find the largest rectangle containing only 1's and return its area.
 * 
 * Example:
 * 
 * 
 * Input:
 * [
 *   ["1","0","1","0","0"],
 *   ["1","0","1","1","1"],
 *   ["1","1","1","1","1"],
 *   ["1","0","0","1","0"]
 * ]
 * Output: 6
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix.first().map_or(0, |r| r.len());
        if rows == 0 || cols == 0 {
            return 0;
        }

        let mut heights = vec![0;cols];
        let mut result = 0;
        for i in 0..rows {
            heights.iter_mut().zip(matrix[i].iter()).for_each(|(a,b)| {
                *a = if b == &'0' { 0 } else { *a + 1 }
            });
            result = result.max(largest_rectangle_area(&heights));
        }

        result
    }
}

fn largest_rectangle_area(heights: &Vec<i32>) -> i32 {
    let len = heights.len();
    if len == 0 {
        return 0;
    }

    // construct the nearest left less array
    let mut left_less_nearest = vec![0i32; len];
    // left bound of start element
    left_less_nearest[0] = -1;
    // right
    let mut right_less_nearest = vec![0i32; len];
    // bound of end element
    *right_less_nearest.last_mut().unwrap() = len as i32;

    for i in 1..len {
        let mut left_bound_index = i as i32 - 1;
        while left_bound_index >= 0 && heights[left_bound_index as usize] >= heights[i] {
            // dp
            left_bound_index = left_less_nearest[left_bound_index as usize];
        }
        left_less_nearest[i] = left_bound_index;
    }



    // we should loop from right
    for i in (0..len-1).rev() {
        let mut right_bound_index = i as i32 + 1;
        while right_bound_index < len as i32 && heights[right_bound_index as usize] >= heights[i] {
            right_bound_index = right_less_nearest[right_bound_index as usize];
        }
        right_less_nearest[i] = right_bound_index;
    }

    let mut result = 0;

    for i in 0..len {
        result = result.max(heights[i] * (right_less_nearest[i] - left_less_nearest[i] - 1) as i32);
    }
    result
}

// submission codes end

#[cfg(test)]
mod tests {
    use crate::*;

    use super::*;

    #[test]
    fn test_85() {
        assert_eq!(Solution::maximal_rectangle(vec2!
        [
  ['1','0','1','0','0'],
  ['1','0','1','1','1'],
  ['1','1','1','1','1'],
  ['1','0','0','1','0']
]
        ), 6);
        assert_eq!(Solution::maximal_rectangle(vec2!
        []
        ), 0);
        assert_eq!(Solution::maximal_rectangle(vec2!
        [['1','1','1','1','1','1','1','1'],['1','1','1','1','1','1','1','0'],['1','1','1','1','1','1','1','0'],['1','1','1','1','1','0','0','0'],['0','1','1','1','1','0','0','0']]
        ), 21);
    }
}
