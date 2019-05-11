/**
 * [66] Plus One
 *
 * Given a non-empty array of digits representing a non-negative integer, plus one to the integer.
 * 
 * The digits are stored such that the most significant digit is at the head of the list, and each element in the array contain a single digit.
 * 
 * You may assume the integer does not contain any leading zero, except the number 0 itself.
 * 
 * Example 1:
 * 
 * 
 * Input: [1,2,3]
 * Output: [1,2,4]
 * Explanation: The array represents the integer 123.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [4,3,2,1]
 * Output: [4,3,2,2]
 * Explanation: The array represents the integer 4321.
 * 
 */
/// reverse, add and carry
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut plus = true;
        for n in digits.iter_mut().rev() {
            if plus {
                *n += 1;
                if *n > 9 {
                    *n = 0;
                } else {
                    plus = false;
                }
            } else {
                break;
            }
        }
        if plus {
            [vec![1], digits].concat()
        } else {
            digits
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_66() {
        assert_eq!(Solution::plus_one(vec![1,2,3]), vec![1,2,4]);
        assert_eq!(Solution::plus_one(vec![4,3,2,1]), vec![4,3,2,2]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1,0]);
        assert_eq!(Solution::plus_one(vec![1,9]), vec![2,0]);
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
        assert_eq!(Solution::plus_one(vec![9,9,9]), vec![1,0,0,0]);
    }
}
