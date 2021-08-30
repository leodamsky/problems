// @leetup=custom
// @leetup=info id=21 lang=rust slug=merge-two-sorted-lists

/*
* Merge two sorted linked lists and return it as a sorted list. The list
* should be made by splicing together the nodes of the first two lists.
* 
* 
* Example 1:
* 
* []
* Input: l1 = [1,2,4], l2 = [1,3,4]
* Output: [1,1,2,3,4,4]
* 
* Example 2:
* 
* Input: l1 = [], l2 = []
* Output: []
* 
* Example 3:
* 
* Input: l1 = [], l2 = [0]
* Output: [0]
* 
* 
* Constraints:
* 
* * The number of nodes in both lists is in the range `[0, 50]`.
* * `-100 <= Node.val <= 100`
* * Both `l1` and `l2` are sorted in non-decreasing order.
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
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
impl Solution {
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_root = ListNode::new(0);
        let mut tmp = &mut dummy_root;

        while let (Some(n1), Some(n2)) = (&l1, &l2) {
            let val = if n1.val <= n2.val {
                let v = n1.val;
                l1 = l1.and_then(|l1| l1.next);
                v
            } else {
                let v = n2.val;
                l2 = l2.and_then(|l2| l2.next);
                v
            };
            tmp.next = Some(Box::new(ListNode::new(val)));
            tmp = tmp.next.as_mut().unwrap();
        }

        let mut remainder = l1.or(l2);
        while let Some(n) = remainder {
            tmp.next = Some(Box::new(ListNode::new(n.val)));
            tmp = tmp.next.as_mut().unwrap();
            remainder = n.next;
        }

        dummy_root.next
    }
}
// @leetup=code

// @leetup=inject:after_code

struct Solution;

// @leetup=inject:after_code
