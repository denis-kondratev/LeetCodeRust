/// # [52. N-Queens II](https://leetcode.com/problems/n-queens-ii/)
///
/// The n-queens puzzle is the problem of placing `n` queens on an `n x n` chessboard such
/// that no two queens attack each other.
///
/// Given an integer `n`, return the number of distinct solutions to the n-queens puzzle.
///
/// ## Example 1:<br>
/// ![image](https://assets.leetcode.com/uploads/2020/11/13/queens.jpg)
/// - Input: `n` = 4
/// - Output: 2
/// - Explanation: There are two distinct solutions to the 4-queens puzzle as shown.
///
/// ## Example 2:
/// - Input: `n` = 1
/// - Output: 1
///
/// ## Constraints:
/// - 1 < = `n` < = 9
pub struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as u16;
        Self::get_queens(n, 0, 0, 0, 0, (1 << n) - 1)
    }

    fn get_queens(max: u16, cur_row: u16, cols: u16, diags1: u16, diags2: u16, mask: u16) -> i32 {
        if cur_row == max {
            return 1;
        }

        let mut count = 0;
        let mut col_bits = mask & !(cols | diags1 | diags2);

        while col_bits > 0 {
            let bit = col_bits & col_bits.wrapping_neg();
            col_bits ^= bit;
            count += Self::get_queens(
                max,
                cur_row + 1,
                cols | bit,
                (diags1 | bit) << 1,
                (diags2 | bit) >> 1,
                mask,
            );
        }

        count
    }
}
