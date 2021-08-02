/*
 * @lc app=leetcode id=716 lang=rust
 *
 * [716] Max Stack
 *
 * https://leetcode.com/problems/max-stack/description/
 *
 * algorithms
 * Easy (43.78%)
 * Total Accepted:    83.9K
 * Total Submissions: 191.6K
 * Testcase Example:  '["MaxStack","push","push","push","top","popMax","top","peekMax","pop","top"]\n[[],[5],[1],[5],[],[],[],[],[],[]]'
 *
 * Design a max stack data structure that supports the stack operations and
 * supports finding the stack's maximum element.
 * 
 * Implement the MaxStack class:
 * 
 * 
 * MaxStack() Initializes the stack object.
 * void push(int x) Pushes element x onto the stack.
 * int pop() Removes the element on top of the stack and returns it.
 * int top() Gets the element on the top of the stack without removing it.
 * int peekMax() Retrieves the maximum element in the stack without removing
 * it.
 * int popMax() Retrieves the maximum element in the stack and removes it. If
 * there is more than one maximum element, only remove the top-most one.
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input
 * ["MaxStack", "push", "push", "push", "top", "popMax", "top", "peekMax",
 * "pop", "top"]
 * [[], [5], [1], [5], [], [], [], [], [], []]
 * Output
 * [null, null, null, null, 5, 5, 1, 5, 1, 5]
 * 
 * Explanation
 * MaxStack stk = new MaxStack();
 * stk.push(5);   // [5] the top of the stack and the maximum number is 5.
 * stk.push(1);   // [5, 1] the top of the stack is 1, but the maximum is 5.
 * stk.push(5);   // [5, 1, 5] the top of the stack is 5, which is also the
 * maximum, because it is the top most one.
 * stk.top();     // return 5, [5, 1, 5] the stack did not change.
 * stk.popMax();  // return 5, [5, 1] the stack is changed now, and the top is
 * different from the max.
 * stk.top();     // return 1, [5, 1] the stack did not change.
 * stk.peekMax(); // return 5, [5, 1] the stack did not change.
 * stk.pop();     // return 1, [5] the top of the stack and the max element is
 * now 5.
 * stk.top();     // return 5, [5] the stack did not change.
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * -10^7 <= x <= 10^7
 * At most 10^4 calls will be made to push, pop, top, peekMax, and popMax.
 * There will be at least one element in the stack when pop, top, peekMax, or
 * popMax is called.
 * 
 * 
 * 
 * Follow up: Could you come up with a solution that supports O(1) for each top
 * call and O(logn) for each other call? 
 */
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

#[derive(Default)]
struct MaxStack {
    idx: usize,
    stack: BTreeMap<usize, i32>,
    maxs: BTreeMap<i32, Vec<usize>>,
    max: Option<i32>,
    top: Option<i32>
}

/*
  `&self` means the method takes an immutable reference.
  If you need a mutable reference, change it to `&mut self` instead.
 */
impl MaxStack {
    /* initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    // O(log(n))
    fn push(&mut self, x: i32) {
        self.maxs.entry(x)
            .or_default()
            .push(self.idx);
        self.stack.insert(self.idx, x);
        self.idx += 1;

        self.top = Some(x);
        self.max = self.maxs.keys().next_back().copied();
    }

    // O(log(n))
    fn pop(&mut self) -> i32 {
        let last_key = *self.stack.keys().next_back().unwrap();
        let x = self.stack.remove(&last_key).unwrap();
        if let Entry::Occupied(mut e) = self.maxs.entry(x) {
            let indices = e.get_mut();
            indices.pop();
            if indices.is_empty() {
                e.remove();
            }
        }
        // FIXME: bounds MaxStack to finite number of operations
        self.idx -= 1;

        self.top = self.stack.iter().next_back().map(|e| *e.1);
        self.max = self.maxs.keys().next_back().map(|e| *e);
        x
    }

    // O(1)
    fn top(&self) -> i32 {
        self.top.unwrap()
    }

    // O(1)
    fn peek_max(&self) -> i32 {
        self.max.unwrap()
    }

    // O(log(n))
    fn pop_max(&mut self) -> i32 {
        let mut entry = match self.maxs.entry(*self.maxs.keys().next_back().unwrap()) {
            Entry::Vacant(_) => unreachable!("not call on empty stack"),
            Entry::Occupied(e) => e
        };
        let indices = entry.get_mut();
        let idx = indices.pop().unwrap();
        let value = self.stack.remove(&idx).unwrap();
        if let Some(new_idx) = indices.last() {
            self.stack.insert(*new_idx, value);
        } else {
            entry.remove();
        }

        self.top = self.stack.iter().next_back().map(|e| *e.1);
        self.max = self.maxs.keys().next_back().map(|e| *e);
        value
    }
}

/*
  Your MaxStack object will be instantiated and called as such:
  let obj = MaxStack::new();
  obj.push(x);
  let ret_2: i32 = obj.pop();
  let ret_3: i32 = obj.top();
  let ret_4: i32 = obj.peek_max();
  let ret_5: i32 = obj.pop_max();
 */
