/// # [144. Binary Tree Preorder Traversal](https://leetcode.com/problems/binary-tree-preorder-traversal/)
///
/// Given the `root` of a binary tree, return the preorder traversal of its nodes' values.
///
/// ## Example 1:
/// - Input: `root` = \[1,null,2,3]
/// - Output: \[1,2,3]
/// - Explanation:<br>
///   ![image](https://assets.leetcode.com/uploads/2024/08/29/screenshot-2024-08-29-202743.png)
///
/// ## Example 2:
/// - Input: `root` = \[1,2,3,4,5,null,8,null,null,6,7,9]
/// - Output: \[1,2,4,5,6,7,3,8,9]
/// - Explanation:<br>
///   ![image](https://assets.leetcode.com/uploads/2024/08/29/tree_2.png)
///
/// ## Example 3:
/// - Input: `root` = \[]
/// - Output: \[]
///
/// ## Example 4:
/// - Input: `root` = \[1]
/// - Output: \[1]
///
/// ## Constraints:
/// - The number of nodes in the tree is in the range \[0, 100].
/// - -100 <= `Node.val` <= 100
///
/// **Follow up:** Recursive solution is trivial, could you do it iteratively?
pub struct Solution;

use binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn preorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = vec![];

        while root.is_some() || !stack.is_empty() {
            while let Some(node) = root {
                result.push(node.borrow().val);
                stack.push(node.clone());
                root = node.borrow().left.clone()
            }

            if let Some(node) = stack.pop() {
                root = node.borrow().right.clone();
            }
        }

        result
    }
}
