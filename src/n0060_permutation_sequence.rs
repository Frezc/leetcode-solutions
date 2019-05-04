/**
 * [60] Permutation Sequence
 *
 * The set [1,2,3,...,n] contains a total of n! unique permutations.
 * 
 * By listing and labeling all of the permutations in order, we get the following sequence for n = 3:
 * 
 * <ol>
 * 	"123"
 * 	"132"
 * 	"213"
 * 	"231"
 * 	"312"
 * 	"321"
 * </ol>
 * 
 * Given n and k, return the k^th permutation sequence.
 * 
 * Note:
 * 
 * 
 * 	Given n will be between 1 and 9 inclusive.
 * 	Given k will be between 1 and n! inclusive.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: n = 3, k = 3
 * Output: "213"
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: n = 4, k = 9
 * Output: "2314"
 *
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    /// e.g. n=3, k=3
    /// 第一位的每个数字都有 (n - 1)! 种可能, 通过 nums[(k - 1) / (n - 1)!] 可以得到第一位数字
    /// 之后通过递归求得 (i..n) 的开头一位就行
    /// 这里要注意nums是剩余数字正序列表, 比如 [1,2,3] 开头取了 2 后, 剩下 [1,3]
    pub fn get_permutation(n: i32, k: i32) -> String {
        if n == 1 {
            return String::from("1");
        }
        let mut result: Vec<i32> = (1..=n).rev().collect();
        let mut index = 0;
        let mut k = k - 1;
        while k > 0 {
            let cur_fac = factorial((n - 1 - index));
            let i = k / cur_fac;
            if i > 0 {
                let splice_index = (n - 1 - index - i) as usize;
                let move_item = result.drain(splice_index..=splice_index).next().unwrap();
                let insert_index = (n-1-index) as usize;
                result.splice(insert_index..insert_index, move_item..=move_item);
            }
            k = k % cur_fac;
            index += 1;
        }
        result.iter().map(|num| num.to_string()).rev().collect()
    }
}

fn factorial(n: i32) -> i32 {
    (1..=n).product()
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_60() {
        assert_eq!(Solution::get_permutation(3, 3), "213");
        assert_eq!(Solution::get_permutation(4, 9), "2314");
        assert_eq!(Solution::get_permutation(1, 1), "1");
        assert_eq!(Solution::get_permutation(2, 2), "21");
        assert_eq!(Solution::get_permutation(9, 362880), "987654321");

    }
}
