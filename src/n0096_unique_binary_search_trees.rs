
use crate::tree::TreeNode;

/**
 * [96] Unique Binary Search Trees
 *
 * Given n, how many structurally unique BST's (binary search trees) that store values 1 ... n?
 * 
 * Example:
 * 
 * 
 * Input: 3
 * Output: 5
 * Explanation:
 * Given n = 3, there are a total of 5 unique BST's:
 * 
 *    1         3     3      2      1
 *     \       /     /      / \      \
 *      3     2     1      1   3      2
 *     /     /       \                 \
 *    2     1         2                 3
 * 
 * 
 */
/// same as 95, but we are not allowed to generate all trees. It will make memory limit exceeded.
pub struct Solution {}

// submission codes start here
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = HashMap::new();
        if n == 0 {
            return 0;
        }
        generate_trees(&mut dp, 1, n)
    }
}

fn generate_trees(dp: &mut HashMap<String, i32>, start: i32, end: i32) -> i32 {
    let key = format!("{}-{}", start, end);
    if let Some(&v) = dp.get(&key) {
        return v;
    }
    if start > end {
        return 1;
    }

    if start == end {
        return 1;
    }

    let mut result = 0;

    for i in start..=end {
        let left_trees = generate_trees(dp, start, i - 1);
        let right_trees = generate_trees(dp, i+1,end);
        result += left_trees * right_trees;
    }
    dp.insert(key, result);
    result
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_96() {
        assert_eq!(Solution::num_trees(3), 5);
        assert_eq!(Solution::num_trees(15), 9694845)
    }
}
