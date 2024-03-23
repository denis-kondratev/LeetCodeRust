/// # [143. Reorder List](https://leetcode.com/problems/reorder-list/description/?envType=daily-question&envId=2024-03-23)
///
/// You are given the head of a singly linked-list. The list can be represented as:
/// - L0 → L1 → … → Ln - 1 → Ln
///
/// Reorder the list to be on the following form:
/// - L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …
///
/// You may not modify the values in the list's nodes. Only nodes themselves may be changed.
///
/// ## Example 1:
/// - Input: head = \[1,2,3,4]
/// - Output: \[1,4,2,3]
///
/// ## Example 2:
/// - Input: head = \[1,2,3,4,5]
/// - Output: \[1,5,2,4,3]
///
/// ## Constraints:
/// - The number of nodes in the list is in the range \[1, 5 * 104].
/// - 1 <= Node.val <= 1000
pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

use std::collections::VecDeque;
use std::ops::DerefMut;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut head = head.as_mut().unwrap();
        let mut deque = VecDeque::new();
        let mut node = match head.next.take() {
            Some(next) => next,
            None => return
        };

        loop {
            let next = node.deref_mut().next.take();
            deque.push_back(node);
            node = match next {
                Some(value) => value,
                None => break
            };
        }

        let mut take_from_front = false;

        while let Some(next) = if take_from_front { deque.pop_front() } else { deque.pop_back() } {
            head.next = Some(next);
            head = head.deref_mut().next.as_mut().unwrap();
            take_from_front = !take_from_front;
        }
    }
}