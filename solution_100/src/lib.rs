/// # [100. Same Tree](https://leetcode.com/problems/same-tree)
///
/// Given the roots of two binary trees `p` and `q`, write a function to check if they
/// are the same or not. Two binary trees are considered the same if they are structurally
/// identical, and the nodes have the same value.
///
/// ## Example 1:
/// ![image](https://assets.leetcode.com/uploads/2020/12/20/ex1.jpg)
/// - Input: `p` = `[1,2,3]`, `q` = `[1,2,3]`
/// - Output: `true`
///
/// ## Example 2:
/// ![image](https://assets.leetcode.com/uploads/2020/12/20/ex2.jpg)
/// - Input: `p` = `[1,2]`, `q` = `[1,null,2]`
/// - Output: `false`
///
/// ## Example 3:
/// ![image](https://assets.leetcode.com/uploads/2020/12/20/ex3.jpg)
/// - Input: `p` = `[1,2,1]`, `q` = `[1,1,2]`
/// - Output: `false`
///
/// ## Constraints:
/// - The number of nodes in both trees is in the range `[0, 100]`.
/// - -10<sup>4</sup> <= Node.val <= 10<sup>4</sup>
pub struct Solution;

use binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                let a = a.borrow();
                let b = b.borrow();

                a.val == b.val
                    && Self::is_same_tree(a.left.clone(), b.left.clone())
                    && Self::is_same_tree(a.right.clone(), b.right.clone())
            }
            _ => false,
        }
    }
}
