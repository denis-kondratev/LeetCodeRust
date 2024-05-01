/// # [58. Length of Last Word](https://leetcode.com/problems/length-of-last-word/description/?envType=daily-question&envId=2024-04-01)
///
/// Given a string `s` consisting of words and spaces, return the length of the last word in
/// the string. A word is a maximal substring consisting of non-space characters only.
///
/// ## Example 1:
/// - Input: `s` = "Hello World"
/// - Output: 5
/// - Explanation: The last word is "World" with length 5.
///
/// ## Example 2:
/// - Input: `s` = "   fly me   to   the moon  "
/// - Output: 4
/// - Explanation: The last word is "moon" with length 4.
///
/// ## Example 3:
/// - Input: `s` = "luffy is still joyboy"
/// - Output: 6
/// - Explanation: The last word is "joyboy" with length 6.
///
/// ## Constraints:
/// - 1 <= `s.length` <= 10<sup>4</sup>
/// - `s     consists of only English letters and spaces ' '.
/// - There will be at least one word in `s`.
pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let word_end = match s.as_bytes().iter().rev().enumerate().find_map(|(i, x)| {
            if *x != b' ' {
                Some(i)
            } else {
                None
            }
        }) {
            None => return 0,
            Some(value) => value,
        };

        match s
            .as_bytes()
            .iter()
            .rev()
            .skip(word_end)
            .enumerate()
            .find_map(|(i, x)| if *x == b' ' { Some(i) } else { None })
        {
            None => (s.len() - word_end) as i32,
            Some(value) => value as i32,
        }
    }
}
