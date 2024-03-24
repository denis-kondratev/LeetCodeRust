/// # [287. Find the Duplicate Number](https://leetcode.com/problems/find-the-duplicate-number/description/?envType=daily-question&envId=2024-03-24)
///
/// Given an array of integers `nums` containing `n + 1` integers where each integer is in
/// the range `[1, n]` inclusive. There is only one repeated number in `nums`, return this
/// repeated number. You must solve the problem without modifying the array `nums` and uses
/// only constant extra space.
///
/// ## Example 1:
/// - Input: `nums` = `[1,3,4,2,2]`
/// - Output: 2
///
/// ## Example 2:
/// - Input: `nums` = `[3,1,3,4,2]`
/// - Output: 3
///
/// ## Example 3:
/// - Input: `nums` = `[3,3,3,3,3]`
/// - Output: 3
///
/// ## Constraints:
/// - 1 <= `n` <= 10<sup>5</sup>
/// - `nums.length` == `n + 1`
/// - 1 <= `nums[i]` <= `n`
/// - All the integers in nums appear only once except for precisely one integer which
/// appears two or more times.
///
/// ## Follow up:
/// - How can we prove that at least one duplicate number must exist in nums?
/// - Can you solve the problem in linear runtime complexity?
pub struct Solution;

impl Solution {
    pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
        let mut num = nums[0] as usize;
        while nums[num] > 0 {
            let next_num = nums[num] as usize;
            nums[num] = 0;
            num = next_num;
        }
        num as i32
    }
}
