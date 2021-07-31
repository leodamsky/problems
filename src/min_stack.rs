// @leetup=custom
// @leetup=info id=155 lang=rust slug=min-stack

/*
* Design a stack that supports push, pop, top, and retrieving the minimum element
* in constant time.
* 
* Implement the `MinStack` class:
* 
* * `MinStack()` initializes the stack object.
* * `void push(val)` pushes the element `val` onto the stack.
* * `void pop()` removes the element on the top of the stack.
* * `int top()` gets the top element of the stack.
* * `int getMin()` retrieves the minimum element in the stack.
* 
* 
* Example 1:
* 
* Input
* ["MinStack","push","push","push","getMin","pop","top","getMin"]
* [[],[-2],[0],[-3],[],[],[],[]]
* Output
* [null,null,null,null,-3,null,0,-2]
* Explanation
* MinStack minStack = new MinStack();
* minStack.push(-2);
* minStack.push(0);
* minStack.push(-3);
* minStack.getMin(); // return -3
* minStack.pop();
* minStack.top();    // return 0
* minStack.getMin(); // return -2
* 
* 
* Constraints:
* 
* * `-231 <= val <= 231 - 1`
* * Methods `pop`, `top` and `getMin` operations will always be called on
*   non-empty stacks.
* * At most `3 * 104` calls will be made to `push`, `pop`, `top`, and `getMin`.
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

struct MinStack {
    items: Vec<i32>,
    min: i32
}

/*
  `&self` means the method takes an immutable reference.
  If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /* initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            items: Vec::new(),
            min: i32::MAX
        }
    }

    // O(1)
    fn push(&mut self, val: i32) {
        self.items.push(val);
        if val < self.min {
            self.min = val;
        }
    }

    // O(n)
    fn pop(&mut self) {
        self.items.pop();
        self.min = self.items.iter().cloned().min().unwrap_or(i32::MAX);
    }

    // O(1)
    fn top(&self) -> i32 {
        *self.items.last().unwrap()
    }

    // O(1)
    fn get_min(&self) -> i32 {
        self.min
    }
}

mod best {
    use std::cmp::Ordering;

    #[derive(Default)]
    struct MinStack {
        stack: Vec<i32>,
        mins: Vec<(i32, usize)>
    }

    impl MinStack {
        fn new() -> Self {
            Default::default()
        }

        fn push(& mut self, x: i32) {
            self.stack.push(x);
            if let Some((min, count)) = self.mins.last_mut() {
                match x.cmp(min) {
                    Ordering::Less => self.mins.push((x, 1)),
                    Ordering::Equal => *count += 1,
                    Ordering::Greater => {}
                }
            } else {
                self.mins.push((x, 1));
            }
        }

        fn pop(&mut self) {
            match (self.stack.pop(), self.mins.last_mut()) {
                (Some(item), Some((min, count))) => {
                    if item == *min {
                        *count -= 1;
                    }
                    if *count == 0 {
                        self.mins.pop();
                    }
                }
                _ => {}
            }
        }

        fn top(&self) -> i32 {
            *self.stack.last().unwrap()
        }

        fn get_min(&self) -> i32 {
            self.mins.last().unwrap().0
        }
    }
}

/*
  Your MinStack object will be instantiated and called as such:
  let obj = MinStack::new();
  obj.push(val);
  obj.pop();
  let ret_3: i32 = obj.top();
  let ret_4: i32 = obj.get_min();
 */
// @leetup=code

// @leetup=inject:after_code

struct Solution;

// @leetup=inject:after_code
