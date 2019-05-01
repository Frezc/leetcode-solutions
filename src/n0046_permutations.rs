/**
 * [46] Permutations
 *
 * Given a collection of distinct integers, return all possible permutations.
 * 
 * Example:
 * 
 * 
 * Input: [1,2,3]
 * Output:
 * [
 *   [1,2,3],
 *   [1,3,2],
 *   [2,1,3],
 *   [2,3,1],
 *   [3,1,2],
 *   [3,2,1]
 * ]
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() > 1 {
            nums.iter().flat_map(|&n|
                Solution::permute(
                    nums.iter()
                        .filter(|&&n1| n1 != n)
                        .map(|&n| n)
                        .collect()
                ).into_iter().map(move |arr| [vec![n], arr].concat())
            ).collect()
        } else if nums.len() == 1 {
            vec![nums]
        } else {
            vec![]
        }
    }
}


// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_46() {
        assert_eq!(Solution::permute(vec![1, 2, 3]), vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1]
        ]);
        assert_eq!(Solution::permute(vec![]), vec![vec![1; 0]; 0]);
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }
}
