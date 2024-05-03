/// # [234. Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list/?envType=daily-question&envId=2024-03-24)
///
/// Given the `head` of a singly linked list, return `true` if it is a palindrome or `false`
/// otherwise.
///
/// ## Example 1:
/// - Input: `head` = `[1,2,2,1]`
/// - Output: `true`
///
/// ## Example 2:
/// - Input: `head` = `[1,2]`
/// - Output: `false`
///
/// ## Constraints:
/// - The number of nodes in the list is in the range \[1, 10<sup>5</sup>].
/// - 0 <= `Node.val` <= 9
///
/// **Follow up:** Could you do it in O(n) time and O(1) space?
pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut vector = Vec::new();
        let mut node = head;

        while let Some(value) = node {
            vector.push(value.val);
            node = value.next;
        }

        let mut left = 0;
        let mut right = vector.len() - 1;

        while left < right {
            if vector[left] != vector[right] {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }
}