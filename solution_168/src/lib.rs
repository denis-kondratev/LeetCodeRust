/// # [168. Excel Sheet Column Title](https://leetcode.com/problems/excel-sheet-column-title/)
///
/// Given an integer `columnNumber`, return its corresponding column title as
/// it appears in an Excel sheet.
///
/// ## For example:
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
/// - Input: `columnNumber` = 1
/// - Output: "A"
///
/// ## Example 2:
/// - Input: `columnNumber` = 28
/// - Output: "AB"
///
/// ## Example 3:
/// -- Input: `columnNumber` = 701
/// -- Output: "ZY"
///
/// ## Constraints:
/// - 1 <= `columnNumber` <= 2<sup>31</sup> - 1
pub struct Solution;

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut deque = std::collections::VecDeque::new();
        const MOD: i32 = (b'Z' - b'A' + 1) as i32;

        while column_number > 0 {
            column_number -= 1;
            let cur = (column_number % MOD) as u8 + b'A';
            column_number /= MOD;
            deque.push_front(cur as char);
        }

        deque.into_iter().collect()
    }
}
