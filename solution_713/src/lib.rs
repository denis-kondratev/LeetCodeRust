/// # [713. Subarray Product Less Than K](https://leetcode.com/problems/subarray-product-less-than-k/description/?envType=daily-question&envId=2024-03-27)
///
/// Given an array of integers `nums` and an integer `k`, return the number of contiguous
/// subarrays where the product of all the elements in the subarray is strictly less than `k`.
///
/// ## Example 1:
/// - Input: `nums` = \[10,5,2,6], `k` = 100
/// - Output: 8
/// - Explanation: The 8 subarrays that have product less than 100 are: \[10], \[5],
/// \[2], \[6], \[10, 5], \[5, 2], \[2, 6], \[5, 2, 6]. Note that \[10, 5, 2] is not included
/// as the product of 100 is not strictly less than `k`.
///
/// ## Example 2:
/// - Input: `nums` = \[1,2,3], `k` = 0
/// - Output: 0
///
/// ## Constraints:
/// - 1 <= `nums.length` <= 3 * 10<sup>4</sup>
/// - 1 <= `nums[i]` <= 1000
/// - 0 <= `k` <= 10<sup>6</sup>
pub struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 { return 0; }
        let (mut left, mut product, mut count) = (0, 1, 0);

        for right in 0..nums.len() {
            product *= nums[right];

            while product >= k {
                product /= nums[left];
                left += 1;
            }

            count += right - left + 1;
        }

        count as i32
    }
}