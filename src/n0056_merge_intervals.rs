/**
 * [56] Merge Intervals
 *
 * Given a collection of intervals, merge all overlapping intervals.
 * 
 * Example 1:
 * 
 * 
 * Input: [[1,3],[2,6],[8,10],[15,18]]
 * Output: [[1,6],[8,10],[15,18]]
 * Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [[1,4],[4,5]]
 * Output: [[1,5]]
 * Explanation: Intervals [1,4] and [4,5] are considered overlapping.
 * 
 * NOTE: input types have been changed on April 15, 2019. Please reset to default code definition to get new method signature.
 * 
 */
/// The key point of this question is to sort array by first element.
/// Then we can check intersection and merge in one loop.
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return intervals;
        }
        // 排好序就很简单了
        intervals.sort_by_key(|item| item[0]);
        let mut result = vec![];
        let mut current_interval = intervals[0].clone();
        for (i, interval) in intervals.iter().enumerate().skip(1) {
            if interval[0] <= current_interval[1] && interval[0] >= current_interval[0] {
                current_interval[0] = current_interval[0].min(interval[0]);
                current_interval[1] = current_interval[1].max(interval[1]);
            } else {
                result.push(current_interval.clone());
                current_interval = intervals[i].clone();
            }
        }
        result.push(current_interval.clone());
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_56() {
        assert_eq!(Solution::merge(vec2![[1,3],[2,6],[8,10],[15,18]]), vec2![[1,6],[8,10],[15,18]]);
        assert_eq!(Solution::merge(vec2![[1,4],[4,5]]), vec2![[1,5]]);
        assert_eq!(Solution::merge(vec![]), vec![vec![0i32; 0]; 0]);
        assert_eq!(Solution::merge(vec2![[1,2],[3,4],[2,3]]), vec2![[1,4]]);
        assert_eq!(Solution::merge(vec2![[1,2],[3,4],[5,6]]), vec2![[1,2],[3,4],[5,6]]);
        assert_eq!(Solution::merge(vec2![[-2232, -1], [888, 999], [666,777], [-2,9]]), vec2![[-2232,9],[666,777],[888,999]]);
    }
}
