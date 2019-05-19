/**
 * [80] Remove Duplicates from Sorted Array II
 *
 * Given a sorted array nums, remove the duplicates <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> such that duplicates appeared at most twice and return the new length.
 * 
 * Do not allocate extra space for another array, you must do this by modifying the input array <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> with O(1) extra memory.
 * 
 * Example 1:
 * 
 * 
 * Given nums = [1,1,1,2,2,3],
 * 
 * Your function should return length = 5, with the first five elements of nums being 1, 1, 2, 2 and 3 respectively.
 * 
 * It doesn't matter what you leave beyond the returned length.
 * 
 * Example 2:
 * 
 * 
 * Given nums = [0,0,1,1,1,1,2,3,3],
 * 
 * Your function should return length = 7, with the first seven elements of nums being modified to 0, 0, 1, 1, 2, 3 and 3 respectively.
 * 
 * It doesn't matter what values are set beyond the returned length.
 * 
 * 
 * Clarification:
 * 
 * Confused why the returned value is an integer but your answer is an array?
 * 
 * Note that the input array is passed in by reference, which means modification to the input array will be known to the caller as well.
 * 
 * Internally you can think of this:
 * 
 * 
 * // nums is passed in by reference. (i.e., without making a copy)
 * int len = removeDuplicates(nums);
 * 
 * // any modification to nums in your function would be known by the caller.
 * // using the length returned by your function, it prints the first len elements.
 * for (int i = 0; i < len; i++) {
 *     print(nums[i]);
 * }
 * 
 * 
 */
///
/// Use two variable saving repeat count & move offset.
/// increase repeat count if num equal to previous or reset if not.
/// increase offset if repeat >= 2.
/// move item to previous by offset in every loop.
///
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut offset = 0;
        let mut repeat = 0;
        for i in 1..nums.len() {
            if nums[i] == nums[i-1] {
                repeat += 1;
                if repeat > 1 {
                    offset += 1;
                    continue;
                }
            } else {
                repeat = 0;
            }
            nums[i - offset] = nums[i];
        }
        (nums.len() - offset) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_80() {
        let mut nums = vec![1,1,1,2,2,3];
        let result = Solution::remove_duplicates(&mut nums) as usize;
        assert_eq!(nums[..result], [1,1,2,2,3]);

        let mut nums = vec![0,0,1,1,1,1,2,3,3];
        let result = Solution::remove_duplicates(&mut nums) as usize;
        assert_eq!(nums[..result], [0, 0, 1, 1, 2, 3, 3]);

        let mut nums = vec![];
        let result = Solution::remove_duplicates(&mut nums) as usize;
        assert_eq!(nums[..result], []);

        let mut nums = vec![1,1,1,1,1];
        let result = Solution::remove_duplicates(&mut nums) as usize;
        assert_eq!(nums[..result], [1, 1]);
    }
}
