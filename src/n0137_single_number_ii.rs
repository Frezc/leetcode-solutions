/**
 * [137] Single Number II
 *
 * Given a non-empty array of integers, every element appears three times except for one, which appears exactly once. Find that single one.
 * 
 * Note:
 * 
 * Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?
 * 
 * Example 1:
 * 
 * 
 * Input: [2,2,3,2]
 * Output: 3
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [0,1,0,1,0,1,99]
 * Output: 99
 * 
 */
/// count 1 at every bits in binary.
/// Be carefully about negative number and -2147483648
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut bit_count = [0usize; 32];

        for num in nums {
            count_bit(&mut bit_count, num);
        }

        let mut re = 0;

        for c in &bit_count[1..] {
            re += re + if c % 3 > 0 { 1 } else { 0 };
        }

        if bit_count[0] % 3 > 0 {
            if re == 0 {
                return -2147483648;
            }
            re = -re;
        }
        re
    }
}

fn count_bit(re: &mut [usize; 32], mut num: i32) {
    if num == -2147483648 {
        re[0] += 1;
        return;
    }
    if num < 0 {
        re[0] += 1;
        num = -num;
    }
    let mut i = 1;
    while num != 0 {
        if num % 2 == 1 {
            re[32 - i] += 1;
        }

        num >>= 1;
        i += 1;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_137() {
        assert_eq!(0, -0);
        assert_eq!(Solution::single_number(vec![2,2,3,2]), 3);
        assert_eq!(Solution::single_number(vec![0,1,0,1,0,1,99]), 99);
        assert_eq!(Solution::single_number(vec![-2,-2,1,1,-3,1,-3,-3,-4,-2]), -4);
        assert_eq!(Solution::single_number(vec![43,16,45,89,45,-2147483648,45,2147483646,-2147483647,-2147483648,43,2147483647,-2147483646,-2147483648,89,-2147483646,89,-2147483646,-2147483647,2147483646,-2147483647,16,16,2147483646,43]), 2147483647);
        assert_eq!(Solution::single_number(vec![-401451,-177656,-2147483646,-473874,-814645,-2147483646,-852036,-457533,-401451,-473874,-401451,-216555,-917279,-457533,-852036,-457533,-177656,-2147483646,-177656,-917279,-473874,-852036,-917279,-216555,-814645,2147483645,-2147483648,2147483645,-814645,2147483645,-216555]), -2147483648);
    }
}
