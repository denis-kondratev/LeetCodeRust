/// # [992. Subarrays with K Different Integers](https://leetcode.com/problems/subarrays-with-k-different-integers/description/?envType=daily-question&envId=2024-03-30)
///
/// Given an integer array `nums` and an integer `k`, return the number of good subarrays of
/// `nums`. A good array is an array where the number of different integers in that array
/// is exactly `k`.
///
/// For example, \[1,2,3,1,2] has 3 different integers: 1, 2, and 3. A subarray is a contiguous
/// part of an array.
///
/// ## Example 1:
/// - Input: `nums` = \[1,2,1,2,3], `k` = 2
/// - Output: 7
/// - Explanation: Subarrays formed with exactly 2 different integers: \[1,2], \[2,1],
/// \[1,2], \[2,3], \[1,2,1], \[2,1,2], \[1,2,1,2]
///
/// ## Example 2:
/// - Input: `nums` = \[1,2,1,3,4], `k` = 3
/// - Output: 3
/// - Explanation: Subarrays formed with exactly 3 different integers: \[1,2,1,3],
/// \[2,1,3], \[1,3,4].
/// å
/// ## Constraints:
/// - 1 <= `nums.length` <= 2 * 10<sup>4</sup>
/// - 1 <= `nums[i]`, `k` <= `nums.length`
pub struct Solution;

use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let sub_with_max_element_k = Self::subarrays_with_k_distinct_intr(&nums, k as usize);
        let reduced_sub_with_max_k = Self::subarrays_with_k_distinct_intr(&nums, k as usize - 1);
        sub_with_max_element_k - reduced_sub_with_max_k
    }

    fn subarrays_with_k_distinct_intr(nums: &Vec<i32>, k: usize) -> i32 {
        let mut map = HashMap::new();
        let (mut count, mut left) = (0, 0);

        for right in 0..nums.len() {
            map.entry(nums[right]).and_modify(|x| *x += 1).or_insert(1);
            while map.len() > k {
                if let Occupied(entry) = map.entry(nums[left]).and_modify(|x| *x -= 1) {
                    if *entry.get() == 0 {
                        entry.remove();
                    }
                }
                left += 1;
            }
            count += right - left + 1;
        }
        count as i32
    }
}
