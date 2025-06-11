/// # [111. Minimum Depth of Binary Tree](https://leetcode.com/problems/minimum-depth-of-binary-tree/)
///
/// Given a binary tree, find its minimum depth.
///
/// The minimum depth is the number of nodes along the shortest path from the root node down
/// to the nearest leaf node.
///
/// Note: A leaf is a node with no children.
///
/// ## Example 1:
/// ![image](https://assets.leetcode.com/uploads/2020/10/12/ex_depth.jpg)
/// - Input: `root` = \[3,9,20,null,null,15,7]
/// - Output: 2
///
/// ## Example 2:
/// - Input: `root` = \[2,null,3,null,4,null,5,null,6]
/// - Output: 5
///
/// ## Constraints:
/// - The number of nodes in the tree is in the range \[0, 10<sup>5</sup>].
/// - -1000 <= `Node.val` <= 1000
pub struct Solution;

use binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };

        let left = Self::min_depth(root.borrow().left.clone());
        let right = Self::min_depth(root.borrow().right.clone());

        if left == 0 {
            return 1 + right;
        } else if right == 0 {
            return 1 + left;
        }

        1 + std::cmp::min(left, right)
    }
}
