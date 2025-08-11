/// # [61. Rotate List](https://leetcode.com/problems/rotate-list/)
///
/// Given the `head` of a linked list, rotate the list to the right by `k` places.
///
/// ## Example 1:
/// ![image](https://assets.leetcode.com/uploads/2020/11/13/rotate1.jpg)
/// - Input: `head` = \[1,2,3,4,5], `k` = 2
/// - Output: \[4,5,1,2,3]
///
/// ## Example 2:
/// ![image](https://assets.leetcode.com/uploads/2020/11/13/roate2.jpg)
/// - Input: `head` = \[0,1,2], `k` = 4
/// - Output: \[2,0,1]
///
/// ## Constraints:
/// - The number of nodes in the list is in the range \[0, 500].
/// - -100 <= `Node.val` <= 100
/// - 0 <= `k` <= 2 * 10<sup>9</sup>
pub struct Solution;

use linked_list::ListNode;

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // Empty list or a single element — nothing to rotate
        if head.is_none() || head.as_ref().unwrap().next.is_none() || k == 0 {
            return head;
        }

        // Calculate the length of the list
        let mut list_len = 0usize;
        {
            let mut cur = head.as_ref();
            while let Some(node) = cur {
                list_len += 1;
                cur = node.next.as_ref();
            }
        }

        let k = (k as usize) % list_len;
        if k == 0 {
            return head;
        }

        // Find the new tail: move (len - k - 1) steps from the old head
        let steps_to_new_tail = list_len - k - 1;
        let mut new_tail = head.as_mut().unwrap();
        for _ in 0..steps_to_new_tail {
            new_tail = new_tail.next.as_mut().unwrap();
        }

        // Detach the right part — it will be the new head
        let mut new_head = new_tail.next.take(); // now the left part ends at new_tail

        // Find the tail of the right part and attach the old head (left part) to it
        {
            let mut cur = new_head.as_mut().unwrap();
            while cur.next.is_some() {
                cur = cur.next.as_mut().unwrap();
            }
            cur.next = head; // attached the old head to the end
        }

        new_head
    }
}