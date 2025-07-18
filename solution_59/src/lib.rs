/// # [59. Spiral Matrix II](https://leetcode.com/problems/spiral-matrix-ii/)
///
/// Given a positive integer `n`, generate an `n x n` matrix filled with elements from
/// 1 to n<sup>2</sup> in spiral order.
///
/// ## Example 1:
/// ![image](https://assets.leetcode.com/uploads/2020/11/13/spiraln.jpg)
/// - Input: n = 3
/// - Output: \[\[1,2,3],\[8,9,4],\[7,6,5]]
///
/// ## Example 2:
/// - Input: n = 1
/// - Output: \[\[1]]
///
/// ## Constraints:
/// - 1 <= n <= 20
pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; n as usize]; n as usize];
        let (mut min, mut max) = (0, n as usize - 1);
        let mut number = 1;

        while min < max {
            for i in min..=max {
                result[min][i] = number;
                number += 1;
            }

            for i in min + 1..=max {
                result[i][max] = number;
                number += 1;
            }

            for i in (min..max).rev() {
                result[max][i] = number;
                number += 1;
            }

            for i in (min + 1..max).rev() {
                result[i][min] = number;
                number += 1;
            }

            min += 1;
            max -= 1;
        }

        if min == max {
            result[min][min] = number;
        }

        result
    }
}
