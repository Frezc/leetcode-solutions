/**
 * [84] Largest Rectangle in Histogram
 *
 * Given n non-negative integers representing the histogram's bar height where the width of each bar is 1, find the area of largest rectangle in the histogram.
 * 
 *  
 * 
 * <img src="https://assets.leetcode.com/uploads/2018/10/12/histogram.png" style="width: 188px; height: 204px;" /><br />
 * <small>Above is a histogram where width of each bar is 1, given height = [2,1,5,6,2,3].</small>
 * 
 *  
 * 
 * <img src="https://assets.leetcode.com/uploads/2018/10/12/histogram_area.png" style="width: 188px; height: 204px;" /><br />
 * <small>The largest rectangle is shown in the shaded area, which has area = 10 unit.</small>
 * 
 *  
 * 
 * Example:
 * 
 * 
 * Input: [2,1,5,6,2,3]
 * Output: 10
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
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
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_84() {
        assert_eq!(Solution::largest_rectangle_area(vec![2,1,5,6,2,3]), 10);
        assert_eq!(Solution::largest_rectangle_area(vec![]), 0);
        assert_eq!(Solution::largest_rectangle_area(vec![1,1]), 2);
        assert_eq!(Solution::largest_rectangle_area(vec![2,3]), 4);
    }
}
