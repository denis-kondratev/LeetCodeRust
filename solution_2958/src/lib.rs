/// # [2958. Length of Longest Subarray With at Most K Frequency](https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency/description/?envType=daily-question&envId=2024-03-28)
///
/// You are given an integer array `nums` and an integer `k`. The frequency of an element `x`
/// is the number of times it occurs in an array. An array is called good if the frequency of
/// each element in this array is less than or equal to `k`. Return the length of the longest
/// good subarray of `nums`.
///
/// A subarray is a contiguous non-empty sequence of elements within an array.
///
/// ## Example 1:
/// - Input: `nums` = \[1,2,3,1,2,3,1,2], `k` = 2
/// - Output: 6
/// - Explanation: The longest possible good subarray is \[1,2,3,1,2,3] since the values
/// 1, 2, and 3 occur at most twice in this subarray. Note that the subarrays
/// \[2,3,1,2,3,1] and \[3,1,2,3,1,2] are also good. It can be shown that there are no good
/// subarrays with length more than 6.
///
/// ## Example 2:
/// - Input: `nums` = \[1,2,1,2,1,2,1,2], `k` = 1
/// - Output: 2
/// - Explanation: The longest possible good subarray is \[1,2] since the values 1 and 2 occur
/// at most once in this subarray. Note that the subarray \[2,1] is also good. It can be shown
/// that there are no good subarrays with length more than 2.
///
/// ## Constraints:
/// - 1 <= `nums.length` <= 10<sup>5</sup>
/// - 1 <= `nums[i]` <= 10<sup>9</sup>
/// - 1 <= `k` <= `nums.length`
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        if k == nums.len() {
            return k as i32;
        }
        let mut map = HashMap::new();
        let mut max_length = 0;
        let mut left= 0;

        for right in 0..nums.len() {
            let right_num = nums[right];
            let right_num_count = map.entry(right_num).or_insert(0);
            *right_num_count += 1;
            let mut num_count = *right_num_count;

            while num_count > k {
                let left_num = nums[left];
                left += 1;
                let left_num_count = map.entry(left_num).or_insert(0);
                *left_num_count -= 1;
                if left_num == right_num {
                    num_count = *left_num_count;
                }
            }

            max_length = max_length.max(right - left + 1);
        }

        max_length as i32
    }
}