/**
 * [162] Find Peak Element
 *
 * A peak element is an element that is strictly greater than its neighbors.
 * Given an integer array nums, find a peak element, and return its index. If the array contains multiple peaks, return the index to any of the peaks.
 * You may imagine that nums[-1] = nums[n] = -&infin;.
 * You must write an algorithm that runs in O(log n) time.
 *  
 * Example 1:
 * 
 * Input: nums = [1,2,3,1]
 * Output: 2
 * Explanation: 3 is a peak element and your function should return the index number 2.
 * Example 2:
 * 
 * Input: nums = [1,2,1,3,5,6,4]
 * Output: 5
 * Explanation: Your function can return either index number 1 where the peak element is 2, or index number 5 where the peak element is 6.
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 1000
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	nums[i] != nums[i + 1] for all valid i.
 * 
 */
pub struct Solution {}

// submission codes start here

///
/// 二分查找，每次找中间的元素判断是否是peak，如果不是找左侧或右侧比它大的一边递归进行查找
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        find_peak_element(&nums, 0) as i32
    }
}

fn find_peak_element(nums: &[i32], start_index: usize) -> usize {
    let len = nums.len();

    let num = nums[len / 2];
    let gp = if len / 2 <= 0 { true } else { num > nums[len / 2 - 1] };
    let gn = if len / 2 + 1 >= len { true } else { num > nums[len / 2 + 1] };
    if gp && gn {
        return start_index + len / 2;
    }
    if !gp {
        return find_peak_element(&nums[..len / 2], start_index);
    }

    find_peak_element(&nums[len / 2 + 1..], start_index + len / 2 + 1)
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_162() {
        assert_eq!(Solution::find_peak_element(vec![1]), 0);
        assert_eq!(Solution::find_peak_element(vec![1,2]), 1);
        assert_eq!(Solution::find_peak_element(vec![3,2]), 0);
        assert_eq!(Solution::find_peak_element(vec![1,2,3]), 2);
        assert_eq!(Solution::find_peak_element(vec![1,3,2]), 1);
    }
}
