/**
 * [77] Combinations
 *
 * Given two integers n and k, return all possible combinations of k numbers out of 1 ... n.
 * 
 * Example:
 * 
 * 
 * Input: n = 4, k = 2
 * Output:
 * [
 *   [2,4],
 *   [3,4],
 *   [2,3],
 *   [1,2],
 *   [1,3],
 *   [1,4],
 * ]
 * 
 * 
 */
/// recursion & backtracing
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if k <= 0 || n <= 0 {
            return vec![];
        }

        let k = k.min(n);
        let mut result = vec![];
        let mut prefix = vec![];
        combine_pre(&mut result, &mut prefix, 1, n, k);

        result
    }
}

fn combine_pre(result: &mut Vec<Vec<i32>>, prefix: &mut Vec<i32>, start: i32, n: i32, k: i32) {
    for i in start..=n-k+1 {
        if k == 1 {
            let mut re = prefix.clone();
            re.push(i);
            result.push(re);
        } else {
            prefix.push(i);
            combine_pre(result, prefix, i+1,n,k-1);
            prefix.pop();
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_77() {
        assert_vec2_eq(&Solution::combine(4, 2), &vec2![
  [2,4],
  [3,4],
  [2,3],
  [1,2],
  [1,3],
  [1,4]
]);
        assert_vec2_eq(&Solution::combine(4, 0), &vec![vec![0;0];0]);
        assert_vec2_eq(&Solution::combine(4, 1), &vec2![[1], [2], [3], [4]]);
    }
}
