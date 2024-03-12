/// # [1171. Remove Zero Sum Consecutive Nodes from Linked List](https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/description/?envType=daily-question&envId=2024-03-12)
///
/// Given the head of a linked list, we repeatedly delete consecutive sequences of nodes
/// that sum to 0 until there are no such sequences. After doing so, return the head of the
/// final linked list.  You may return any such answer.
///
/// (Note that in the examples below, all sequences are serializations of ListNode objects.)
///
/// ## Example 1:
/// - Input: head = \[1,2,-3,3,1]
/// - Output: \[3,1]
/// - Note: The answer \[1,2,1] would also be accepted.
///
/// ## Example 2:
/// - Input: head = \[1,2,3,-3,4]
/// - Output: \[1,2,4]
///
/// ## Example 3:
/// - Input: head = \[1,2,3,-3,-2]
/// - Output: \[1]
///
/// ## Constraints:
/// - The given linked list will contain between 1 and 1000 nodes.
/// - Each node in the linked list has -1000 <= node.val <= 1000.
pub struct Solution;

impl Solution {
    pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Move list values into an array
        let mut list = vec![];
        while let Some(node) = head {
            list.push(node.val);
            head = node.next;
        }

        // Remove zero-sum sequences from array
        let mut start = 0;
        while start < list.len() {
            let mut prefix = 0;
            let mut end = start;
            while end < list.len() {
                prefix += list[end];
                if prefix == 0 {
                    list.drain(start..=end);
                    start -= 1;
                    break;
                }
                end += 1;
            }
            start += 1;
        }

        // Create new list
        let mut new = ListNode::new(0);
        let mut n = &mut new;
        for val in list.into_iter().map(ListNode::new).map(Box::new) {
            n.next = Some(val);
            n = n.next.as_mut()?;
        }
        n.next = None;
        new.next
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
