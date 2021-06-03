/**
 * [587] Erect the Fence
 *
 * You are given an array trees where trees[i] = [xi, yi] represents the location of a tree in the garden.
 * You are asked to fence the entire garden using the minimum length of rope as it is expensive. The garden is well fenced only if all the trees are enclosed.
 * Return the coordinates of trees that are exactly located on the fence perimeter.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/erect2-plane.jpg" style="width: 509px; height: 500px;" />
 * Input: points = [[1,1],[2,2],[2,0],[2,4],[3,3],[4,2]]
 * Output: [[1,1],[2,0],[3,3],[2,4],[4,2]]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/erect1-plane.jpg" style="width: 509px; height: 500px;" />
 * Input: points = [[1,2],[2,2],[4,2]]
 * Output: [[4,2],[2,2],[1,2]]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= points.length <= 3000
 * 	points[i].length == 2
 * 	0 <= xi, yi <= 100
 * 	All the given points are unique.
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        vec![]

    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_587() {
        assert_eq!(Solution::outer_trees(vec2![[1,1],[2,2],[2,0],[2,4],[3,3],[4,2]]), vec2![[1,1],[2,0],[3,3],[2,4],[4,2]]);
        assert_eq!(Solution::outer_trees(vec2![[1,2],[2,2],[4,2]]), vec2![[4,2],[2,2],[1,2]]);
    }
}
