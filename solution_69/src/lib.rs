/// # [69. Sqrt(x)](https://leetcode.com/problems/sqrtx/)
///
/// Given a non-negative integer `x`, return the square root of `x` rounded down to the
/// nearest integer. The returned integer should be non-negative as well.
///
/// You must not use any built-in exponent function or operator.
///
/// For example, do not use `pow(x, 0.5)` in c++ or `x ** 0.5` in python.
///
/// ## Example 1:
/// - Input: `x = 4`
/// - Output: `2`
/// - Explanation: The square root of `4` is `2`, so we return `2`.
///
/// ## Example 2:
/// - Input: `x = 8`
/// - Output: `2`
/// - Explanation: The square root of `8` is `2.82842...`, and since we round it down to the
///   nearest integer, `2` is returned.
///
/// ## Constraints:
/// - `0` <= `x` <= `231 - 1`
pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }

        let mut left = 1;
        let mut right = x / 2;
        let mut ans = 0;

        while left <= right {
            let mid = left + (right - left) / 2;
            if mid <= x / mid {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        ans
    }
}
