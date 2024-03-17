/// # [1. Two Sum](https://leetcode.com/problems/two-sum/description/)
///
/// Given an array of integers nums and an integer target, return indices of the two numbers
/// such that they add up to target. You may assume that each input would have exactly one
/// solution, and you may not use the same element twice.
///
/// You can return the answer in any order.
///
/// ## Example 1:
/// - Input: nums = \[2,7,11,15], target = 9
/// - Output: \[0,1]
/// - Explanation: Because nums\[0] + nums\[1] == 9, we return \[0, 1].
///
/// ## Example 2:
/// - Input: nums = \[3,2,4], target = 6
/// - Output: \[1,2]
///
/// ## Example 3:
/// - Input: nums = \[3,3], target = 6
/// - Output: \[0,1]
///
/// ## Constraints:
/// - 2 <= `nums.length` <= 10<sup>4</sup>
/// - -10<sup>9</sup> <= nums\[i] <= 10<sup>9</sup>
/// - -10<sup>9</sup> <= target <= 10<sup>9</sup>
/// - Only one valid answer exists.
///
/// **Follow-up:** Can you come up with an algorithm that is less than O(n2) time complexity?
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (i, a) in nums.iter().enumerate() {
            let b = target - a;

            if let Some(&j) = map.get(&b) {
                return vec![i as i32, j as i32];
            }

            map.insert(*a, i);
        }

        vec![-1, -1]
    }
}
