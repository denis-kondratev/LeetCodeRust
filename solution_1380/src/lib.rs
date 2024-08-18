/// # [1380. Lucky Numbers in a Matrix](https://leetcode.com/problems/lucky-numbers-in-a-matrix/description/?envType=daily-question&envId=2024-07-19)
///
/// Given an m x n matrix of distinct numbers, return all lucky numbers in the matrix in any order.
/// A lucky number is an element of the matrix such that it is the minimum element in its
/// row and maximum in its column.
///
/// ## Example 1:
/// - Input: `matrix` = \[\[3,7,8],\[9,11,13],\[15,16,17]]
/// - Output: \[15]
/// - Explanation: 15 is the only lucky number since it is the minimum in its row and the
///   maximum in its column.
///
/// ## Example 2:
/// - Input: matrix = \[\[1,10,4,2],\[9,3,8,7],\[15,16,17,12]]
/// - Output: \[12]
/// - Explanation: 12 is the only lucky number since it is the minimum in its row and the
///   maximum in its column.
///
/// ## Example 3:
/// - Input: matrix = \[\[7,8],\[1,2]]
/// - Output: \[7]
/// - Explanation: 7 is the only lucky number since it is the minimum in its row and the maximum
///   in its column.
///
/// ## Constraints:
/// - `m` == `mat.length`
/// - `n` == `mat[i].length`
/// - `1` <= `n, m` <= `50`
/// - `1` <= `matrix[i][j]` <= `105`.
/// - All elements in the matrix are distinct.
pub struct Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (n, m) = (matrix.len(), matrix[0].len());
        let mut mins = vec![0i32; m];
        let mut maxs = vec![0i32; m];

        for i in 0..n {
            let mut min = i32::MAX;
            let mut min_j = 0;
            for j in 0..m {
                let num = matrix[i][j];

                if num < min {
                    min = num;
                    min_j = j;
                }

                if num > maxs[j] {
                    maxs[j] = num;
                }
            }

            if mins[min_j] < min {
                mins[min_j] = min;
            }
        }

        let mut result = vec![];
        for i in 0..m {
            if mins[i] == maxs[i] {
                result.push(mins[i]);
            }
        }

        result
    }
}
