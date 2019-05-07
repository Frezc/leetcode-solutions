/**
 * [48] Rotate Image
 *
 * You are given an n x n 2D matrix representing an image.
 * 
 * Rotate the image by 90 degrees (clockwise).
 * 
 * Note:
 * 
 * You have to rotate the image <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a>, which means you have to modify the input 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.
 * 
 * Example 1:
 * 
 * 
 * Given input matrix = 
 * [
 *   [1,2,3],
 *   [4,5,6],
 *   [7,8,9]
 * ],
 * 
 * rotate the input matrix in-place such that it becomes:
 * [
 *   [7,4,1],
 *   [8,5,2],
 *   [9,6,3]
 * ]
 * 
 * 
 * Example 2:
 * 
 * 
 * Given input matrix =
 * [
 *   [ 5, 1, 9,11],
 *   [ 2, 4, 8,10],
 *   [13, 3, 6, 7],
 *   [15,14,12,16]
 * ], 
 * 
 * rotate the input matrix in-place such that it becomes:
 * [
 *   [15,13, 2, 5],
 *   [14, 3, 4, 1],
 *   [12, 6, 8, 9],
 *   [16, 7,10,11]
 * ]
 * 
 * 
 */
/// Loop every 4-corner-point-rotate circle by circle.
/// # Example
/// [
///   [1,2,3],
///   [4,5,6],
///   [7,8,9]
/// ],
/// 1. first rotate 4 corner element
/// [
///   [7,2,1],
///   [4,5,6],
///   [9,8,3]
/// ],
/// 2. then rotate every element on every edge
/// [
///   [7,4,1],
///   [8,5,2],
///   [9,6,3]
/// ],
/// 3. rotate next circle inside
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        if matrix.len() != 0 {
            let l = matrix.len();
            for i in 0..l / 2 {
                for j in 0..l - 2 * i - 1 {
                    let c = matrix[i + j][l - 1 - i];
                    matrix[i + j][l - 1 - i] = matrix[i][i + j];
                    matrix[i][i + j] = matrix[l - 1 - i - j][i];
                    matrix[l - 1 - i - j][i] = matrix[l - 1 - i][l - 1 - i - j];
                    matrix[l - 1 - i][l - 1 - i - j] = c;
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_48() {
        let mut matrix1 = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        Solution::rotate(&mut matrix1);
        assert_eq!(matrix1, vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ]);

        let mut matrix2 = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix2);
        assert_eq!(matrix2, vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11]
        ]);

        let mut matrix3 = vec![vec![0; 0]; 0];
        Solution::rotate(&mut matrix3);
        assert_eq!(matrix3, vec![vec![0; 0]; 0]);

        let mut matrix4 = vec![
            vec![13],
        ];
        Solution::rotate(&mut matrix4);
        assert_eq!(matrix4, vec![
            vec![13],
        ]);
    }
}
