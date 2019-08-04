/**
 * [128] Longest Consecutive Sequence
 *
 * Given an unsorted array of integers, find the length of the longest consecutive elements sequence.
 * 
 * Your algorithm should run in O(n) complexity.
 * 
 * Example:
 * 
 * 
 * Input: [100, 4, 200, 1, 3, 2]
 * Output: 4
 * Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
 * 
 * 
 */
/// It obvious we should sort array first. To meet O(n) complexity, we could choose radix sort.
/// This question has some edge cases that isn't mentioned:
/// 1. Negative numbers. We need sort negative and positive separately in radix sort.
/// 2. Repeated numbers. Example: for input [1,1,2,2,3,3], result is [1,2,3]
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 { return nums.len() as i32; }
        let mut pos = vec![];
        let mut neg = vec![];
        nums.iter().for_each(|&num| {
            if num >= 0 {
                pos.push(num);
            } else {
                neg.push(-num);
            }
        });

        let sorted_pos = radix_sort(&pos);
        let mut sorted_neg = radix_sort(&neg);
        sorted_neg.reverse();
        let nums = [sorted_neg.into_iter().map(|num| -num).collect(), sorted_pos].concat();
        let mut max = 1;
        let mut cur = 1;
        for i in 1..nums.len() {
            if nums[i] - nums[i-1] == 1 {
                cur += 1;
                max = max.max(cur);
            } else if nums[i] != nums[i-1] {
                cur = 1;
            }
        }

        max
    }
}

fn radix_sort(nums: &[i32]) -> Vec<i32> {
    let mut buckets = vec![vec![0;0];10];
    let mut temp = Vec::from(nums);

    let mut radix = 0;
    loop {
        // prevent overflow, loop 10 times
        if radix >= 10 {
            break;
        }

        for num in temp {
            buckets[num as usize / 10usize.pow(radix) % 10].push(num);
        }

        temp = buckets.into_iter().fold(Vec::with_capacity(nums.len()), |acc, cur| {
            [acc, cur].concat()
        });

        buckets = vec![vec![0;0];10];
        radix += 1;
    }

    temp
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_128() {
        assert_eq!(radix_sort(&[2147483647,80,12,11,9,101]), vec![9,11,12,80,101,2147483647]);
        assert_eq!(radix_sort(&[100, 4, 200, 1, 3, 2]), vec![1,2,3,4,100,200]);
        assert_eq!(Solution::longest_consecutive(vec![]), 0);
        assert_eq!(Solution::longest_consecutive(vec![0,0]), 1);
        assert_eq!(Solution::longest_consecutive(vec![0,-1]), 2);
        assert_eq!(Solution::longest_consecutive(vec![1,2,0,1]), 3);
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }
}
