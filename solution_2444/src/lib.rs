/// # [2444. Count Subarrays With Fixed Bounds](https://leetcode.com/problems/count-subarrays-with-fixed-bounds/description/?envType=daily-question&envId=2024-03-31)
///
/// You are given an integer array `nums` and two integers `minK` and `maxK`.
/// A fixed-bound subarray of `nums` is a subarray that satisfies the following conditions:
///
/// - The minimum value in the subarray is equal to `minK`.
/// - The maximum value in the subarray is equal to `maxK`.
///
/// Return the number of fixed-bound subarrays. A subarray is a contiguous part of an array.
///
/// ## Example 1:
/// - Input: `nums` = \[1,3,5,2,7,5], `minK` = 1, `maxK` = 5
/// - Output: 2
/// - Explanation: The fixed-bound subarrays are \[1,3,5] and \[1,3,5,2].
///
/// ## Example 2:
/// - Input: `nums` = \[1,1,1,1], `minK` = 1, `maxK` = 1
/// - Output: 10
/// - Explanation: Every subarray of `nums` is a fixed-bound subarray. There are 10
/// possible subarrays.
///
/// ## Constraints:
/// - 2 <= `nums.length` <= 10<sup>5</sup>
/// - 1 <= `nums[i]`, `minK`, `maxK` <= 10<sup>6</sup>
pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let (mut count, mut left): (i64, i64) = (0, -1);
        let (mut prev_max_k_idx, mut prev_min_k_idx): (i64, i64) = (-1, -1);

        for (right, &num) in nums.iter().enumerate() {
            let right = right as i64;
            if num < min_k || num > max_k {
                left = right;
            } else {
                prev_min_k_idx = if num == min_k { right } else { prev_min_k_idx };
                prev_max_k_idx = if num == max_k { right } else { prev_max_k_idx };
            }

            count += (prev_max_k_idx.min(prev_min_k_idx) - left).max(0);
        }

        count
    }
}
