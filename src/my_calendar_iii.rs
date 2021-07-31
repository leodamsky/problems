// @leetup=custom
// @leetup=info id=732 lang=rust slug=my-calendar-iii

/*
* A `k`-booking happens when `k` events have some non-empty intersection (i.e.,
* there is some time that is common to all `k` events.)
* 
* You are given some events `[start, end)`, after each given event, return an
* integer `k` representing the maximum `k`-booking between all the previous
* events.
* 
* Implement the `MyCalendarThree` class:
* 
* * `MyCalendarThree()` Initializes the object.
* * `int book(int start, int end)` Returns an integer `k` representing the largest
*   integer such that there exists a `k`-booking in the calendar.
* 
* 
* Example 1:
* 
* Input
* ["MyCalendarThree", "book", "book", "book", "book", "book", "book"]
* [[], [10, 20], [50, 60], [10, 40], [5, 15], [5, 10], [25, 55]]
* Output
* [null, 1, 1, 2, 3, 3, 3]
* Explanation
* MyCalendarThree myCalendarThree = new MyCalendarThree();
* myCalendarThree.book(10, 20); // return 1, The first event can be booked and is 
* disjoint, so the maximum k-booking is a 1-booking.
* myCalendarThree.book(50, 60); // return 1, The second event can be booked and is
*  disjoint, so the maximum k-booking is a 1-booking.
* myCalendarThree.book(10, 40); // return 2, The third event [10, 40) intersects t
* he first event, and the maximum k-booking is a 2-booking.
* myCalendarThree.book(5, 15); // return 3, The remaining events cause the maximum
*  K-booking to be only a 3-booking.
* myCalendarThree.book(5, 10); // return 3
* myCalendarThree.book(25, 55); // return 3
* 
* 
* Constraints:
* 
* * `0 <= start < end <= 109`
* * At most `400` calls will be made to `book`.
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

use std::collections::BTreeMap;

#[derive(Default)]
struct MyCalendarThree {
    tree: BTreeMap<i32, i32>,
}

// `&self` means the method takes an immutable reference.
// If you need a mutable reference, change it to `&mut self` instead.
impl MyCalendarThree {
    fn new() -> Self {
        Default::default()
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        *self.tree.entry(start).or_insert(0) += 1;
        *self.tree.entry(end).or_insert(0) -= 1;
        let mut active = 0;
        // this is the main idea
        // it performs an ordered run with overlapping bookings
        let mut res = 0;
        for &v in self.tree.values() {
            active += v;
            res = res.max(active);
        }
        res
    }
}

// Your MyCalendarThree object will be instantiated and called as such:
// let obj = MyCalendarThree::new();
// let ret_1: i32 = obj.book(start, end);
// @leetup=code

// @leetup=inject:after_code

struct Solution;

// @leetup=inject:after_code
