/// # [104. Maximum Depth of Binary Tree](https://leetcode.com/problems/maximum-depth-of-binary-tree)
///
/// Given the `root` of a binary tree, return its maximum depth.
///
/// A binary tree's maximum depth is the number of nodes along the longest path from the root
/// node down to the farthest leaf node.
///
/// ## Example 1:
/// - Input: `root = [3,9,20,null,null,15,7]`
/// - Output: `3`
///
/// ## Example 2:
/// - Input: `root = [1,null,2]`
/// - Output: `2`
///
/// ## Constraints:
/// - The number of nodes in the tree is in the range \[0, 10<sup>4</sup>].
/// - `-100 < = Node.val < = 100`
pub struct Solution;

use binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };

        let root = root.borrow();

        1 + std::cmp::max(
            Self::max_depth(root.left.clone()),
            Self::max_depth(root.right.clone())
        )
    }
}
