/**
 * [57] Insert Interval
 *
 * Given a set of non-overlapping intervals, insert a new interval into the intervals (merge if necessary).
 * 
 * You may assume that the intervals were initially sorted according to their start times.
 * 
 * Example 1:
 * 
 * 
 * Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
 * Output: [[1,5],[6,9]]
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
 * Output: [[1,2],[3,10],[12,16]]
 * Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
 * 
 * NOTE: input types have been changed on April 15, 2019. Please reset to default code definition to get new method signature.
 * 
 */
/// Find the first intersected interval & the last intersected interval,
/// and merge them into one interval.
pub struct Solution {}

// submission codes start here

impl Solution {
    // why this problem is hard?
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut merge_interval = new_interval;
        let mut inserted = false;
        let mut result = vec![];
        for interval in intervals {
            if !inserted && interval[0] > merge_interval[1] {
                inserted = true;
                result.push(merge_interval.clone());
            }
            if inserted || interval[0] > merge_interval[1] || interval[1] < merge_interval[0] {
                result.push(interval);
            } else {
                merge_interval[0] = merge_interval[0].min(interval[0]);
                merge_interval[1] = merge_interval[1].max(interval[1]);
            }
        }
        if !inserted {
            result.push(merge_interval);
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
    fn test_57() {
        assert_eq!(Solution::insert(vec2![[1,3],[6,9]], vec![2,5]), vec2![[1,5],[6,9]]);
        assert_eq!(Solution::insert(vec2![[1,2],[3,5],[6,7],[8,10],[12,16]], vec![4,8]), vec2![[1,2],[3,10],[12,16]]);
        assert_eq!(Solution::insert(vec2![], vec![2,5]), vec2![[2,5]]);
        assert_eq!(Solution::insert(vec2![[1,3],[6,9]], vec![2,2]), vec2![[1,3],[6,9]]);
        assert_eq!(Solution::insert(vec2![[1,3],[6,9]], vec![4,5]), vec2![[1,3],[4,5],[6,9]]);
        assert_eq!(Solution::insert(vec2![[1,3],[6,9]], vec![-1,10]), vec2![[-1, 10]]);
    }
}
