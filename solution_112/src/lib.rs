/// # [112. Path Sum](https://leetcode.com/problems/path-sum/)
///
/// Given the `root` of a binary tree and an integer `targetSum`, return `true` if the tree has
/// a root-to-leaf path such that adding up all the values along the path equals `targetSum`.
///
/// A leaf is a node with no children.
///
/// ## Example 1:
/// ![image](https://assets.leetcode.com/uploads/2021/01/18/pathsum1.jpg)
/// - Input: `root` = \[5,4,8,11,null,13,4,7,2,null,null,null,1], `targetSum` = 22
/// - Output: `true`
/// - Explanation: The root-to-leaf path with the target sum is shown.
///
/// ## Example 2:
/// ![image](https://assets.leetcode.com/uploads/2021/01/18/pathsum2.jpg)
/// - Input: `root` = \[1,2,3], `targetSum` = 5
/// - Output: `false`
/// - Explanation: There are two root-to-leaf paths in the tree:
///   - (1 --> 2): The sum is 3.
///   - (1 --> 3): The sum is 4.
///   - There is no root-to-leaf path with sum = 5.
///
/// ## Example 3:
/// - Input: `root` = \[], `targetSum` = 0
/// - Output: `false`
/// - Explanation: Since the tree is empty, there are no root-to-leaf paths.
///
/// ## Constraints:
/// - The number of nodes in the tree is in the range \[0, 5000].
/// - -1000 <= `Node.val` <= 1000
/// - -1000 <= `targetSum` <= 1000
pub struct Solution;

use binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, mut target_sum: i32) -> bool {
        let Some(root) = root else {
            return false;
        };

        let root = root.borrow();
        target_sum -= root.val;

        if root.left.is_none() && root.right.is_none() {
            return target_sum == 0;
        }

        Self::has_path_sum(root.left.clone(), target_sum)
            || Self::has_path_sum(root.right.clone(), target_sum)
    }
}
