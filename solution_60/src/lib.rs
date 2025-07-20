/// #  [60. Permutation Sequence](https://leetcode.com/problems/permutation-sequence/)
///
/// The set \[1, 2, 3, ..., n] contains a total of n! unique permutations.
///
/// By listing and labeling all the permutations in order, we get the following sequence
/// for `n` = 3:
/// 1. "123"
/// 2. "132"
/// 3. "213"
/// 4. "231"
/// 5. "312"
/// 6. "321"
///
/// Given `n` and `k`, return the k<sup>th</sup> permutation sequence.
///
/// ## Example 1:
/// - Input: `n` = 3, `k` = 3
/// - Output: "213"
///
/// ## Example 2:
/// - Input: `n` = 4, `k` = 9
/// - Output: "2314"
///
/// ## Example 3:
/// - Input: `n` = 3, `k` = 1
/// - Output: "123"
///
/// ## Constraints:
/// - 1 <= `n` <= 9
/// - 1 <= `k` <= n!
pub struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut numbers: Vec<u8> = (1..=n).map(|x| b'0' + x as u8).collect();
        let mut k = k - 1;
        let mut result = Vec::with_capacity(n as usize);
        let mut factorial = (1..n).product::<i32>();

        for i in (1..n).rev() {
            let idx = (k / factorial) as usize;
            result.push(numbers.remove(idx));
            k %= factorial;
            if i > 1 {
                factorial /= i;
            }
        }
        result.push(numbers[0]);
        String::from_utf8(result).unwrap()
    }
}
