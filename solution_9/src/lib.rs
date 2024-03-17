/// # [9. Palindrome Number](https://leetcode.com/problems/palindrome-number/description/)
///
/// Given an integer x, return true if x is a palindrome, and false otherwise.
///
/// ## Example 1:
/// - Input: x = 121
/// - Output: true
/// - Explanation: 121 reads as 121 from left to right and from right to left.
///
/// ## Example 2:
/// - Input: x = -121
/// - Output: false
/// - Explanation: From left to right, it reads -121. From right to left, it becomes 121-.
///   Therefore, it is not a palindrome.
///
/// ## Example 3:
/// - Input: x = 10
/// - Output: false
/// - Explanation: Reads 01 from right to left. Therefore, it is not a palindrome.
///
/// ## Constraints:
/// - -2<sup>31</sup> <= x <= 2<sup>31</sup> - 1
///
/// **Follow up:** Could you solve it without converting the integer to a string?
pub struct Solution;

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 {
            return false;
        }

        if x == 0 {
            return true;
        }

        let mut digits = [0u8; 10];
        let mut size = 0;

        while x > 0 {
            digits[size] = (x % 10) as u8;
            x /= 10;
            size += 1;
        }

        let (mut left, mut right) = (0, size - 1);

        while left < right {
            if digits[left] != digits[right] {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }
}
