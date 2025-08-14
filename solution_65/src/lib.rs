/// # [65. Valid Number](https://leetcode.com/problems/valid-number/)
///
/// Given a string `s`, return whether `s` is a valid number. For example, all the following
/// are valid numbers: "2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7",
/// "+6e-1", "53.5e93", "-123.456e789", while the following are not valid numbers: "abc", "1a",
/// "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53".
///
/// Formally, a valid number is defined using one of the following definitions:
/// 1. An integer number followed by an optional exponent.
/// 2. A decimal number followed by an optional exponent.
///
/// An integer number is defined with an optional sign '-' or '+' followed by digits.
/// A decimal number is defined with an optional sign '-' or '+' followed by one of
/// the following definitions:
/// 1. Digits followed by a dot '.'.
/// 2. Digits followed by a dot '.' followed by digits.
/// 3. A dot '.' followed by digits.
///
/// An exponent is defined with an exponent notation 'e' or 'E' followed by an integer number.
///
/// The digits are defined as one or more digits.
///
/// ## Example 1:
/// - Input: s = "0"
/// - Output: true
///
/// ## Example 2:
/// - Input: s = "e"
/// - Output: false
///
/// ## Example 3:
/// - Input: s = "."
/// - Output: false
///
/// ## Constraints:
/// - 1 <= s.length <= 20
/// - `s` consists of only English letters (both uppercase and lowercase), digits (0-9), plus
///   '+', minus '-', or dot '.'.
pub struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        if s.is_empty() {
            return false;
        }

        let bytes = s.as_bytes();
        let mut seen_digit = false;
        let mut seen_dot = false;
        let mut seen_exp = false;
        let mut digit_after_exp = true; // becomes relevant only after we see an exponent

        let mut i = 0;
        while i < bytes.len() {
            let c = bytes[i];
            match c {
                b'0'..=b'9' => {
                    seen_digit = true;
                    if seen_exp { digit_after_exp = true; }
                }
                b'+' | b'-' => {
                    // Sign is only allowed at the start or immediately after an exponent
                    if i != 0 && bytes[i - 1] != b'e' && bytes[i - 1] != b'E' {
                        return false;
                    }
                }
                b'.' => {
                    // Dot is not allowed after an exponent and only once overall
                    if seen_dot || seen_exp {
                        return false;
                    }
                    seen_dot = true;
                }
                b'e' | b'E' => {
                    // Exponent must appear once and only after at least one digit
                    if seen_exp || !seen_digit {
                        return false;
                    }
                    seen_exp = true;
                    digit_after_exp = false; // require at least one digit after exponent
                }
                _ => return false,
            }
            i += 1;
        }

        // Valid if we've seen a digit overall, and if exponent exists then we also saw a digit after it
        seen_digit && (!seen_exp || digit_after_exp)
    }
}