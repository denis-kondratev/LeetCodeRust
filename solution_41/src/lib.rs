/// # [41. First Missing Positive](https://leetcode.com/problems/first-missing-positive/description/?envType=daily-question&envId=2024-03-26)
///
/// Given an unsorted integer array `nums`. Return the smallest positive integer that is
/// not present in `nums`. You must implement an algorithm that runs in O(n) time and uses
/// O(1) auxiliary space.
///
/// ## Example 1:
/// - Input: `nums` = \[1,2,0]
/// - Output: 3
/// - Explanation: The numbers in the range \[1,2] are all in the array.
///
/// ## Example 2:
/// - Input: `nums` = \[3,4,-1,1]
/// - Output: 2
/// - Explanation: 1 is in the array but 2 is missing.
///
/// ## Example 3:
/// - Input: `nums` = \[7,8,9,11,12]
/// - Output: 1
/// - Explanation: The smallest positive integer 1 is missing.
///
/// ## Constraints:
/// - 1 <= `nums.length` <= 10<sup>5</sup>
/// - -2<sup>31</sup> <= `nums[i]` <= 2<sup>31</sup> - 1
pub struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let mut i = 0;
        while i < nums.len() {
            let x = nums[i];
            let j = x as usize - 1;
            if x > 0 && j < nums.len() && i != j && nums[i] != nums[j] {
                nums.swap(i, j);
            } else {
                i += 1;
            }
        }

        let result = nums.iter().enumerate().find_map(|(i, &x)| {
            let i = i as i32 + 1;
            if i != x {
                Some(i)
            } else {
                None
            }
        });

        if let Some(value) = result {
            value
        } else {
            nums.len() as i32 + 1
        }
    }
}
