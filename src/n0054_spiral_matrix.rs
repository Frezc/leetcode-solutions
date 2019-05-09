/**
 * [54] Spiral Matrix
 *
 * Given a matrix of m x n elements (m rows, n columns), return all elements of the matrix in spiral order.
 * 
 * Example 1:
 * 
 * 
 * Input:
 * [
 *  [ 1, 2, 3 ],
 *  [ 4, 5, 6 ],
 *  [ 7, 8, 9 ]
 * ]
 * Output: [1,2,3,6,9,8,7,4,5]
 * 
 * 
 * Example 2:
 * 
 * Input:
 * [
 *   [1, 2, 3, 4],
 *   [5, 6, 7, 8],
 *   [9,10,11,12]
 * ]
 * Output: [1,2,3,4,8,12,11,10,9,5,6,7]
 * 
 */
/// Simulate the spiral path circle by circle.
/// Pay attention to one width or height matrix. Don't step duplicate element.
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        spiral_order_in_bound(&matrix, Bound {
            t: 0,
            l: 0,
            b: matrix.len(),
            r: matrix.first().map_or(0, |v| v.len())
        })
    }
}

struct Bound {
    t: usize,
    l: usize,
    b: usize,
    r: usize,
}

impl Bound {
    fn size(&self) -> usize {
        if self.b > self.t && self.r > self.l {
            (self.b - self.t) * (self.r - self.l)
        } else {
            0
        }
    }

    fn shrink(mut self) -> Bound {
        self.l += 1;
        self.t += 1;
        self.r -= 1;
        self.b -= 1;
        self
    }
}

fn spiral_order_in_bound(matrix: &Vec<Vec<i32>>, bound: Bound) -> Vec<i32> {
    let size = bound.size();
    if size > 0 {
        let mut result = Vec::with_capacity(size);
        for l in bound.l..bound.r {
            result.push(matrix[bound.t][l]);
        }
        for t in bound.t + 1..bound.b {
            result.push(matrix[t][bound.r - 1]);
        }
        if bound.b - bound.t > 1 {
            for l in (bound.l..bound.r - 1).rev() {
                result.push(matrix[bound.b - 1][l]);
            }
        }
        if bound.r - bound.l > 1 {
            for t in (bound.t + 1..bound.b - 1).rev() {
                result.push(matrix[t][bound.l]);
            }
        }
        result.extend(spiral_order_in_bound(matrix, bound.shrink()));
        return result;
    }
    vec![]
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_54() {
        assert_eq!(Solution::spiral_order(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
        assert_eq!(Solution::spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12]
        ]), vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
        assert_eq!(Solution::spiral_order(vec![]), vec![]);
        assert_eq!(Solution::spiral_order(vec![vec![5]]), vec![5]);
        assert_eq!(Solution::spiral_order(vec![
            vec![5],
            vec![6],
            vec![7],
        ]), vec![5, 6, 7]);
        assert_eq!(Solution::spiral_order(vec![
            vec![5, 6, 7],
        ]), vec![5, 6, 7]);
        assert_eq!(Solution::spiral_order(vec![
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ]), vec![]);
    }
}
