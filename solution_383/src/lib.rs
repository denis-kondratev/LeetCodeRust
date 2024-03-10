/// # [383. Ransom Note](https://leetcode.com/problems/ransom-note/description/?envType=study-plan-v2&envId=top-interview-150)
///
/// Given two strings `ransomNote` and `magazine`, return `true` if `ransomNote` can be
/// constructed by using the letters from `magazine` and `false` otherwise. Each letter in
/// `magazine` can only be used once in `ransomNote`.
///
/// ## Example 1:
/// - Input: `ransomNote` = "a", `magazine` = "b"
/// - Output: `false`
///
/// ## Example 2:
/// - Input: `ransomNote` = "aa", `magazine` = "ab"
/// - Output: `false`
///
/// ## Example 3:
/// - Input: `ransomNote` = "aa", `magazine` = "aab"
/// - Output: `true`
///
/// ## Constraints:
/// - 1 <= `ransomNote.length`, `magazine.length` <= 10<sup>5</sup>
/// - `ransomNote` and `magazine` consist of lowercase English letters.
pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map = magazine
            .as_bytes()
            .iter()
            .fold([0; 26], |mut map, &x| {
                let i = (x - b'a') as usize;
                map[i] += 1;
                map
            });

        for &x in ransom_note.as_bytes().iter() {
            let i = (x - b'a') as usize;
            map[i] -= 1;

            if map[i] < 0 {
                return false;
            }
        }

        true
    }
}
