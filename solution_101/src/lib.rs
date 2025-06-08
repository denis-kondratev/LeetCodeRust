/// # [101. Symmetric Tree](https://leetcode.com/problems/symmetric-tree)
///
/// Given the `root` of a binary tree, check whether it is a mirror of itself
/// (i.e., symmetric around its center).
///
/// ## Example 1:
/// ![image](https://assets.leetcode.com/uploads/2021/02/19/symtree1.jpg)
/// - Input: `root` = `[1,2,2,3,4,4,3]`
/// - Output: `true`
///
/// ## Example 2:
/// ![image](https://assets.leetcode.com/uploads/2021/02/19/symtree2.jpg)
/// - Input: `root` = `[1,2,2,null,3,null,3]`
/// - Output: `false`
///
/// ## Constraints:
/// - The number of nodes in the tree is in the range `[1, 1000]`.
/// - `-100 <= Node.val <= 100`
///
/// **Follow up:** Could you solve it both recursively and iteratively?
pub struct Solution;

use binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_symmetric(root: Node) -> bool {
        Self::are_symmetric(&root, &root)
    }

    fn are_symmetric(left: &Node, right: &Node) -> bool {
        match (left, right) {
            (Some(left), Some(right)) => {
                left.borrow().val == right.borrow().val
                    && Self::are_symmetric(&left.borrow().left, &right.borrow().right)
                    && Self::are_symmetric(&left.borrow().right, &right.borrow().left)
            }
            (None, None) => true,
            (_, _) => false,
        }
    }
}
