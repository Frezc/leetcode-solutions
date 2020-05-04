
/**
 * [149] Max Points on a Line
 *
 * Given n points on a 2D plane, find the maximum number of points that lie on the same straight line.
 * 
 * Example 1:
 * 
 * 
 * Input: [[1,1],[2,2],[3,3]]
 * Output: 3
 * Explanation:
 * ^
 * |
 * |        o
 * |     o
 * |  o  
 * +------------->
 * 0  1  2  3  4
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
 * Output: 4
 * Explanation:
 * ^
 * |
 * |  o
 * |     o        o
 * |        o
 * |  o        o
 * +------------------->
 * 0  1  2  3  4  5  6
 * 
 * 
 * NOTE: input types have been changed on April 15, 2019. Please reset to default code definition to get new method signature.
 * 
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;

impl Solution {
    /// 基本思路是计算每两个点所在的直线y=ax+b，拿(a,b)为key，出现次数为value来计数。最后拿到出现最多的直线次数+1即为点数。
    /// 不过这题直接拿斜率当作key会有精度问题，需要其他表示方法。
    /// 在解法中我们可以一个点一个点来计算它和其他所有点组成的直线，这时只需要计算斜率即可。因为经过一个点且斜率相同的直线就是同一条直线。
    /// 那么问题就变成了如何用整数代表斜率，最直接的方法就是用分数的分子和分母组成key，(y1-y2)/(x1-x2)。
    /// 为了得到分数的最简表示方式，还需要计算一下最大公约数，可以复习一下辗转相除法。
    /// 最后要考虑一下点重叠的特殊情况，这种时候所有结果都需要+1。
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() < 2 {
            return points.len() as i32;
        }
        let mut max = 2;
        for (i, p1) in points.iter().take(points.len() - 1).enumerate() {
            let mut line_count = HashMap::new();
            let mut overlap = 0;
            let mut p_max = 1;
            for p2 in points.iter().skip(i + 1) {
                let x = p1[0] - p2[0];
                let y = p1[1] - p2[1];
                if x == 0 && y == 0 {
                    overlap += 1;
                    continue;
                }
                let gcd = calc_gcd(p1[1] - p2[1], p1[0] - p2[0]);
                let x = (p1[0] - p2[0]) / gcd;
                let y = (p1[1] - p2[1]) / gcd;
                let count = line_count.entry(x).or_insert(HashMap::new()).entry(y).or_insert(1);
                *count += 1;
                if *count > p_max { p_max = *count }
            }
            if p_max + overlap > max {
                max = p_max + overlap;
            }
        }
        max
    }
}

fn calc_gcd(a: i32, b: i32) -> i32 {
    if b == 0 { return a }
    calc_gcd(b, a % b)
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_149() {
        assert_eq!(Solution::max_points(vec2![[1,1],[2,2],[3,3]]), 3);
        assert_eq!(Solution::max_points(vec2![[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]), 4);
        assert_eq!(Solution::max_points(vec2![]), 0);
        assert_eq!(Solution::max_points(vec2![[0,0]]), 1);
        assert_eq!(Solution::max_points(vec2![[0,0],[0,0],[0,0]]), 3);
        assert_eq!(Solution::max_points(vec2![[0,0],[0,0],[1,0], [2,0],[3,0]]), 5);
    }
}
