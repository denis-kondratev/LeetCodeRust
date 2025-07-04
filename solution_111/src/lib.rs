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

        let mut nodes = std::collections::VecDeque::from([(root, 1)]);

        while let Some((node, depth)) = nodes.pop_front() {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                return depth;
            }
            
            if let Some(left) = &node.left {
                nodes.push_back((left.clone(), depth + 1));
            }
            if let Some(right) = &node.right {
                nodes.push_back((right.clone(), depth + 1));
            }
        }

        unreachable!()
    }
}
