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
        let mut mins = vec![0; n];
        let mut maxs = vec![0; m];

        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] < matrix[i][mins[i]] {
                    mins[i] = j;
                }

                if matrix[i][j] > matrix[maxs[j]][j] {
                    maxs[j] = i;
                }
            }
        }

        let mut result = vec![];
        for i in 0..n {
            if maxs[mins[i]] == i {
                result.push(matrix[i][mins[i]]);
            }
        }

        result
    }
}
