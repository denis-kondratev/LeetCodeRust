/// # [1669. Merge In Between Linked Lists](https://leetcode.com/problems/merge-in-between-linked-lists/description/?envType=daily-question&envId=2024-03-20)
///
/// You are given two linked lists: `list1` and `list2` of sizes `n` and `m` respectively.
/// Remove `list1`'s nodes from the ath node to the bth node, and put `list2` in their place.
/// Build the result list and return its head.
///
/// ## Example 1:
/// - Input: `list1` = \[10,1,13,6,9,5], `a` = 3, `b` = 4, `list2` = \[1000000,1000001,1000002]
/// - Output: \[10,1,13,1000000,1000001,1000002,5]
/// - Explanation: We remove the nodes 3 and 4 and put the entire `list2` in their place.
///
/// ## Example 2:
/// - Input: `list1` = \[0,1,2,3,4,5,6], `a` = 2, `b` = 5, `list2` = \[1000000,1000001,1000002,1000003,1000004]
/// - Output: \[0,1,1000000,1000001,1000002,1000003,1000004,6]
///
/// ## Constraints:
/// - 3 <= `list1.length` <= 10<sup>4</sup>
/// - 1 <= `a` <= `b` < `list1.length` - 1
/// - 1 <= `list2.length` <= 10<sup>4</sup>
pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

trait NodeManager {
    fn skip_as_mut_ref(&mut self, count: i32) -> Option<&mut Self>;
    fn skip(self, count: i32) -> Option<Self> where Self: Sized;
    fn get_last_as_mut_ref(&mut self) -> &mut Self;
}

impl NodeManager for Box<ListNode> {
    fn skip_as_mut_ref(&mut self, count: i32) -> Option<&mut Self> {
        let mut current = self;
        for _ in 0..count {
            current = current.next.as_mut()?;
        }
        Some(current)
    }

    fn skip(self, count: i32) -> Option<Self> {
        let mut current = self;
        for _ in 0..count {
            current = current.next?;
        }
        Some(current)
    }

    fn get_last_as_mut_ref(&mut self) -> &mut Self {
        let mut node = self;
        while let Some(ref mut next) = node.next {
            node = next;
        }
        node
    }
}

impl Solution {
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head1 = list1?;
        let mut head2 = list2?;
        let last2 = head2.get_last_as_mut_ref();
        let a_node = head1.skip_as_mut_ref(a - 1)?;
        let next_a = std::mem::take(&mut a_node.next)?;
        let b_node = next_a.skip(b - a + 1)?;
        last2.next = Some(b_node);
        a_node.next = Some(head2);
        Some(head1)
    }
}
