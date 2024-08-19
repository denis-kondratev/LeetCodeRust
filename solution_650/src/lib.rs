/// # [650. 2 Keys Keyboard](https://leetcode.com/problems/2-keys-keyboard/description/?envType=daily-question&envId=2024-08-19)
///
/// There is only one character 'A' on the screen of a notepad. You can perform one of
/// two operations on this notepad for each step:
///
/// - Copy All: You can copy all the characters present on the screen
///   (a partial copy is not allowed).
/// - Paste: You can paste the characters which are copied last time.
///
/// Given an integer n, return the minimum number of operations to get the character
/// 'A' exactly n times on the screen.
///
/// ## Example 1:
/// - Input: n = 3
/// - Output: 3
/// - Explanation:
///   - Initially, we have one character 'A'.
///   - In step 1, we use Copy All operation.
///   - In step 2, we use Paste operation to get 'AA'.
///   - In step 3, we use Paste operation to get 'AAA'.
///
/// ## Example 2:
/// - Input: n = 1
/// - Output: 0
///
/// ## Constraints:
/// - 1 <= n <= 1000
pub struct Solution;

impl Solution {
    const PRIMES: [i32; 62] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293,
    ];

    pub fn min_steps(n: i32) -> i32 {
        if n == 1 {
            return 0;
        }

        let n_half = n / 2;

        for prime in Self::PRIMES {
            if prime > n_half {
                break;
            }

            if n % prime == 0 {
                return prime + Self::min_steps(n / prime);
            }
        }

        n
    }
}
