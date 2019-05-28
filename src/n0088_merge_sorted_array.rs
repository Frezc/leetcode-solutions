/**
 * [88] Merge Sorted Array
 *
 * Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as one sorted array.
 * 
 * Note:
 * 
 * 
 * 	The number of elements initialized in nums1 and nums2 are m and n respectively.
 * 	You may assume that nums1 has enough space (size that is greater or equal to m + n) to hold additional elements from nums2.
 * 
 * 
 * Example:
 * 
 * 
 * Input:
 * nums1 = [1,2,3,0,0,0], m = 3
 * nums2 = [2,5,6],       n = 3
 * 
 * Output: [1,2,2,3,5,6]
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut mi = m - 1;
        let mut ni = n - 1;
        for i in (0..m+n).rev() {

            if ni < 0 {
                break;
            }
            if mi < 0 || nums2[ni as usize] > nums1[mi as usize] {
                nums1[i as usize] = nums2[ni as usize];
                ni-=1;
            } else {
                nums1[i as usize] = nums1[mi as usize];
                mi-=1;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_88() {
        let mut nums = vec![1,2,3,0,0,0];
        Solution::merge(&mut nums, 3, &mut vec![2,5,6],3);
        assert_eq!(nums, vec![1,2,2,3,5,6]);
    }
}
