/// # [171. Excel Sheet Column Number](https://leetcode.com/problems/excel-sheet-column-number/)
///
/// Given a string `columnTitle` that represents the column title as appears in an Excel sheet,
/// return its corresponding column number.
///
/// For example:
/// - A -> 1
/// - B -> 2
/// - C -> 3
/// - ...
/// - Z -> 26
/// - AA -> 27
/// - AB -> 28
/// - ...
///
/// ## Example 1:
/// - Input: `columnTitle` = "A"
/// - Output: 1
///
/// ## Example 2:
/// - Input: `columnTitle` = "AB"
/// - Output: 28
///
/// ## Example 3:
/// - Input: `columnTitle` = "ZY"
/// - Output: 701
///
/// ## Constraints:
/// - 1 <= `columnTitle`.length <= 7
/// - `columnTitle` consists only of uppercase English letters.
/// - `columnTitle` is in the range \["A", "FXSHRXW"].
pub struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut result = 0;
        const MOD: i32 = (b'Z' - b'A') as i32 + 1;
        let mut multiplier = 1;

        for c in column_title.chars().rev() {
            let value = (c as i32 - 'A' as i32) + 1;
            result += value * multiplier;
            multiplier *= MOD;
        }

        result
    }
}
