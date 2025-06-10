/// #[110. Balanced Binary Tree](https://leetcode.com/problems/balanced-binary-tree/)
///
/// Given a binary tree, determine if it is height-balanced.
///
/// A height-balanced binary tree is a binary tree in which the depth of the two subtrees
/// of every node never differs by more than one.
///
/// ## Example 1:
/// ![image](https://assets.leetcode.com/uploads/2020/10/06/balance_1.jpg)
/// - Input: `root` = \[3,9,20,null,null,15,7]
/// - Output: `true`
///
/// ## Example 2:
/// ![image](https://assets.leetcode.com/uploads/2020/10/06/balance_2.jpg)
/// - Input: `root` = \[1,2,2,3,3,null,null,4,4]
/// - Output: `false`
///
/// ## Example 3:
/// - Input: `root` = \[]
/// - Output: `true`
///
/// ## Constraints:
/// - The number of nodes in the tree is in the range \[0, 5000].
/// - -10<sup>4</sup> <= `Node.val` <= 10<sup>4</sup>
pub struct Solution;

use binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::get_balance_size(root) >= 0
    }

    fn get_balance_size(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };

        let left_size = Self::get_balance_size(root.borrow().left.clone());
        let right_size = Self::get_balance_size(root.borrow().right.clone());

        if left_size == -1 || right_size == -1 || (left_size - right_size).abs() > 1 {
            return -1;
        }

        1 + left_size.max(right_size)
    }
}
