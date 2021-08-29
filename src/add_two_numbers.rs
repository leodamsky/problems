// @leetup=custom
// @leetup=info id=2 lang=rust slug=add-two-numbers

/*
* You are given two non-empty linked lists representing two non-negative
* integers. The digits are stored in reverse order, and each of their nodes
* contains a single digit. Add the two numbers and return the sum as a linked
* list.
*
* You may assume the two numbers do not contain any leading zero, except the
* number 0 itself.
*
*
* Example 1:
*
* []
* Input: l1 = [2,4,3], l2 = [5,6,4]
* Output: [7,0,8]
* Explanation: 342 + 465 = 807.
*
* Example 2:
*
* Input: l1 = [0], l2 = [0]
* Output: [0]
*
* Example 3:
*
* Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
* Output: [8,9,9,9,0,0,0,1]
*
*
* Constraints:
*
* * The number of nodes in each linked list is in the range `[1, 100]`.
* * `0 <= Node.val <= 9`
* * It is guaranteed that the list represents a number that does not have leading
*   zeros.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// Test comment
// Test code
// @leetup=inject:before_code_ex

// @leetup=code

// @leetup=inject:before_code

// @leetup=inject:before_code

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() || l2.is_none() {
            return l1.or(l2);
        }
        const EXP: i32 = 10;

        let mut sum = l1.as_ref().unwrap().as_ref().val + l2.as_ref().unwrap().as_ref().val;
        let mut remainder = sum / EXP;

        let mut root = ListNode::new(sum % EXP);
        let mut tmp = &mut root;

        l1 = l1.and_then(|l1| l1.next);
        l2 = l2.and_then(|l2| l2.next);

        while l1.is_some() || l2.is_some() {
            sum = remainder
                + l1.as_ref().map(|l1| l1.val).unwrap_or(0)
                + l2.as_ref().map(|l2| l2.val).unwrap_or(0);
            tmp.next = Some(Box::new(ListNode::new(sum % EXP)));
            tmp = tmp.next.as_mut().unwrap().as_mut();
            remainder = sum / EXP;

            l1 = l1.and_then(|l1| l1.next);
            l2 = l2.and_then(|l2| l2.next);
        }
        if remainder > 0 {
            tmp.next = Some(Box::new(ListNode::new(remainder)));
        }

        Some(Box::new(root))
    }
}
// @leetup=code

// @leetup=inject:after_code

struct Solution;

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

// @leetup=inject:after_code
