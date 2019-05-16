/**
 * [75] Sort Colors
 *
 * Given an array with n objects colored red, white or blue, sort them <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> so that objects of the same color are adjacent, with the colors in the order red, white and blue.
 * 
 * Here, we will use the integers 0, 1, and 2 to represent the color red, white, and blue respectively.
 * 
 * Note: You are not suppose to use the library's sort function for this problem.
 * 
 * Example:
 * 
 * 
 * Input: [2,0,2,1,1,0]
 * Output: [0,0,1,1,2,2]
 * 
 * Follow up:
 * 
 * 
 * 	A rather straight forward solution is a two-pass algorithm using counting sort.<br />
 * 	First, iterate the array counting number of 0's, 1's, and 2's, then overwrite array with total number of 0's, then 1's and followed by 2's.
 * 	Could you come up with a one-pass algorithm using only constant space?
 * 
 * 
 */
/// Three pointer solution.
/// The first i point to  end of '0' series.
/// The second j point to start of '2' series.
/// The last k point used to iterate.
/// If k point to 0, swap with i.
/// If k point to 2, swap with j.
/// If k point to 1, move to next.
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }

        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut k = i;

        loop {
            if k > j {
                return;
            }

            if nums[k] < 1 && i != k {
                nums.swap(i, k);
                i += 1;
            } else if nums[k] > 1 && k != j {
                nums.swap(k, j);
                j -= 1;
            } else {
                k += 1;
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
    fn test_75() {
        let mut vec = vec![2,0,2,1,1,0];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, vec![0,0,1,1,2,2]);

        vec = vec![];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, vec![]);

        vec = vec![0,0,0,0];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, vec![0,0,0,0]);

        vec = vec![0,2,2,0];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, vec![0,0,2,2]);

        vec = vec![2,1];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, vec![1,2]);

        vec = vec![1, 2,0];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, vec![0,1,2]);

        vec = vec![0,1];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, vec![0,1]);
    }
}
