/// # [119. Pascal's Triangle II](https://leetcode.com/problems/pascals-triangle-ii/)
///
/// Given an integer `rowIndex`, return the `rowIndex`<sup>th</sup> (0-indexed)
/// row of the Pascal's triangle. In Pascal's triangle, each number is the sum of the
/// two numbers directly above it as shown:
///
/// ![image](https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif)
///
/// ## Example 1:
/// - Input: `rowIndex` = 3
/// - Output: \[1, 3, 3, 1]
///
/// ## Example 2:
/// - Input: `rowIndex` = 0
/// - Output: \[1]
///
/// ## Example 3:
/// - Input: `rowIndex` = 1
/// - Output: \[1, 1]
///
/// ## Constraints:
/// - 0 < = `rowIndex` < = 33
///
/// **Follow up:** Could you optimize your algorithm to use only `O(rowIndex)` extra space?
pub struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let n = row_index as i64;
        let mut row = Vec::with_capacity(row_index as usize + 1);

        for k in 0..=n {
            let Some(last) = row.last().copied() else {
                row.push(1);
                continue;
            };

            row.push((last as i64 * (n - k + 1) / k) as i32);
        }

        row
    }
}
