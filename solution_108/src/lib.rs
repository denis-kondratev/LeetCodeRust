/// # [108. Convert Sorted Array to Binary Search Tree](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/)
/// Given an integer array `nums` where the elements are sorted in ascending order,
/// convert it to a height-balanced binary search tree.
///
/// A height-balanced binary tree is a binary tree in which the depth of the two subtrees
/// of every node never differs by more than one.
///
/// ## Example 1:
/// ![image](https://assets.leetcode.com/uploads/2021/02/18/btree1.jpg)
/// - Input: `nums` = \[-10,-3,0,5,9]
/// - Output: \[0,-3,9,-10,null,5]
/// - Explanation: \[0,-10,5,null,-3,null,9] is also accepted:<br>
/// ![image](https://assets.leetcode.com/uploads/2021/02/18/btree2.jpg)
///
/// ## Example 2:
/// ![image](https://assets.leetcode.com/uploads/2021/02/18/btree.jpg)
/// - Input: `nums` = \[1,3]
/// - Output: \[3,1]
/// - Explanation: \[1,null,3] and \[3,1] are both height-balanced BSTs.
///
/// ## Constraints:
/// - 1 <= `nums.length` <= 10<sup>4</sup>
/// - -10<sup>4</sup> <= `nums[i]` <= 10<sup>4</sup>
/// - `nums` is sorted in a strictly increasing order.
pub struct Solution;

use binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_bst(&nums)
    }

    fn build_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let mid = nums.len() / 2;
        let root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));

        root.borrow_mut().left = Self::build_bst(&nums[..mid]);
        root.borrow_mut().right = Self::build_bst(&nums[mid + 1..]);

        Some(root)
    }
}
