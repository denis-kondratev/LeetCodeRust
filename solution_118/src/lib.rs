/// # [118. Pascal's Triangle](https://leetcode.com/problems/pascals-triangle/)
///
/// Given an integer `numRows`, return the first `numRows` of Pascal's triangle.
///
/// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
///
/// ## Example 1:
/// ![image](https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif)
/// - Input: `numRows` = 5
/// - Output: `[[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]`
///
/// ## Example 2:
/// - Input: numRows = 1
/// - Output: `[[1]]`
///
/// ## Constraints:
/// - 1 <= `numRows` <= 30
pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle = Vec::with_capacity(num_rows as usize);
        triangle.push(vec![1]);

        for _ in 1..num_rows {
            let Some(last) = triangle.last() else {
                unreachable!()
            };

            let mut new_row = Vec::with_capacity(last.len() + 2);
            new_row.push(1);

            for pair in last.windows(2) {
                let [a, b] = pair else { unreachable!() };
                new_row.push(a + b);
            }
            new_row.push(1);
            triangle.push(new_row);
        }

        triangle
    }
}
