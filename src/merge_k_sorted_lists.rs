// @leetup=custom
// @leetup=info id=23 lang=rust slug=merge-k-sorted-lists

/*
* You are given an array of `k` linked-lists `lists`, each linked-list is sorted
* in ascending order.
* 
* *Merge all the linked-lists into one sorted linked-list and return it.*
* 
* 
* Example 1:
* 
* Input: lists = [[1,4,5],[1,3,4],[2,6]]
* Output: [1,1,2,3,4,4,5,6]
* Explanation: The linked-lists are:
* [
*   1->4->5,
*   1->3->4,
*   2->6
* ]
* merging them into one sorted list:
* 1->1->2->3->4->4->5->6
* 
* Example 2:
* 
* Input: lists = []
* Output: []
* 
* Example 3:
* 
* Input: lists = [[]]
* Output: []
* 
* 
* Constraints:
* 
* * `k == lists.length`
* * `0 <= k <= 10^4`
* * `0 <= lists[i].length <= 500`
* * `-10^4 <= lists[i][j] <= 10^4`
* * `lists[i]` is sorted in ascending order.
* * The sum of `lists[i].length` won't exceed `10^4`.
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

use std::cmp::Ordering;

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
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        fn go(lists: &mut [Option<Box<ListNode>>]) -> Option<Box<ListNode>> {
            if lists.len() == 1 {
                let mut replacements = vec![None];
                lists.swap_with_slice(&mut replacements);
                return replacements.pop().flatten();
            } else if lists.is_empty() {
                return None;
            }
            let (left, right) = lists.split_at_mut(lists.len() / 2);
            let left = go(left);
            let right = go(right);
            Solution::merge(left, right)
        }
        go(&mut lists)
    }

    fn merge(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(l1), Some(l2)) => {
                let mut nodes = Vec::new();

                let mut l1 = Some(l1);
                let mut l2 = Some(l2);
                while let (Some(node1), Some(node2)) = (&l1, &l2) {
                    match node1.val.cmp(&node2.val) {
                        Ordering::Less => {
                            let node1 = l1.unwrap();
                            nodes.push(ListNode::new(node1.val));
                            l1 = node1.next;
                        }
                        Ordering::Equal => {
                            let node1 = l1.unwrap();
                            nodes.push(ListNode::new(node1.val));
                            l1 = node1.next;
                            let node2 = l2.unwrap();
                            nodes.push(ListNode::new(node2.val));
                            l2 = node2.next;
                        }
                        Ordering::Greater => {
                            let node2 = l2.unwrap();
                            nodes.push(ListNode::new(node2.val));
                            l2 = node2.next;
                        }
                    }
                }

                let mut to_append = l1.or(l2);
                while let Some(node) = to_append {
                    nodes.push(ListNode::new(node.val));
                    to_append = node.next;
                }

                let mut res = nodes.pop().unwrap();
                for mut n in nodes.into_iter().rev() {
                    n.next = Some(Box::new(res));
                    res = n;
                }
                Some(Box::new(res))
            }
            (l1, l2) => l1.or(l2)
        }
    }
}

mod best {
    use std::{cmp::Reverse, collections::BinaryHeap};
    use crate::merge_k_sorted_lists::ListNode;

    impl PartialOrd for ListNode {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.val.partial_cmp(&other.val)
        }
    }

    impl Ord for ListNode {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.val.cmp(&other.val)
        }
    }

    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut min_heap = BinaryHeap::new();
        for list in lists {
            if let Some(node) = list {
                min_heap.push(Reverse(node));
            }
        }
        let mut dummy_head = ListNode::new(0);
        let mut cur = &mut dummy_head;
        while let Some(Reverse(node)) = min_heap.pop() {
            cur.next = Some(Box::new(ListNode::new(node.val)));
            cur = cur.next.as_mut().unwrap();
            if let Some(next) = node.next {
                min_heap.push(Reverse(next));
            }
        }
        return dummy_head.next;
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
        ListNode {
            next: None,
            val,
        }
    }
}
// @leetup=inject:after_code
