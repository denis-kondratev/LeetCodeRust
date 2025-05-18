mod list_node;
use list_node::ListNode;

/// # [83. Remove Duplicates from Sorted List](https://leetcode.com/problems/remove-duplicates-from-sorted-list/)
///
/// Given the head of a sorted linked list, delete all duplicates such that each element
/// appears only once. Return the linked list sorted as well.
///
/// ## Example 1:
/// - Input: head = `[1,1,2]`
/// - Output: `[1, 2]`
///
/// ## Example 2:
/// - Input: head = `[1, 1, 2, 3, 3]`
/// - Output: `[1, 2, 3]`
///
/// ## Constraints:
/// - The number of nodes in the list is in the range `[0, 300]`.
/// - `-100` <= `Node.val` <= `100`
/// - The list is guaranteed to be sorted in ascending order.
pub struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current_opt = head.as_mut();

        while let Some(current) = current_opt {
            Self::delete_following_duplicates(current);
            current_opt = current.next.as_mut();
        }

        head
    }

    fn delete_following_duplicates(node: &mut Box<ListNode>) {
        while let Some(next) = node.next.as_mut() {
            if node.val != next.val {
                break;
            }

            node.next = next.next.take();
        }
    }
}
