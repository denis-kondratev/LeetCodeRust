/// # [264. Ugly Number II](https://leetcode.com/problems/ugly-number-ii/description/?envType=daily-question&envId=2024-08-18)
///
/// An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.
/// Given an integer n, return the nth ugly number.
///
/// ## Example 1:
/// - Input: n = 10
/// - Output: 12
/// - Explanation: \[1, 2, 3, 4, 5, 6, 8, 9, 10, 12] is the sequence of the first 10 ugly numbers.
///
/// ## Example 2:
/// - Input: n = 1
/// - Output: 1
/// - Explanation: 1 has no prime factors, therefore all of its prime factors are
///   limited to 2, 3, and 5.
///
/// ## Constraints:
/// - 1 <= n <= 1690
pub struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut ugly_numbers = vec![1];
        let primes = [2, 3, 5];
        let mut ugly_candidates = [2, 3, 5];
        let mut prime_indices = [1, 1, 1];

        for _ in 1..n {
            let ugly = ugly_candidates.into_iter().min().unwrap();
            ugly_numbers.push(ugly);

            for i in 0..3 {
                if ugly == ugly_candidates[i] {
                    ugly_candidates[i] = ugly_numbers[prime_indices[i]] * primes[i];
                    prime_indices[i] += 1;
                }
            }
        }

        *ugly_numbers.last().unwrap()
    }
}
