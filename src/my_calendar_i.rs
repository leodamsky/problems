// @leetup=custom
// @leetup=info id=729 lang=rust slug=my-calendar-i

/*
* You are implementing a program to use as your calendar. We can add a new event
* if adding the event will not cause a double booking.
*
* A double booking happens when two events have some non-empty intersection
* (i.e., some moment is common to both events.).
*
* The event can be represented as a pair of integers `start` and `end` that
* represents a booking on the half-open interval `[start, end)`, the range of real
* numbers `x` such that `start <= x < end`.
*
* Implement the `MyCalendar` class:
*
* * `MyCalendar()` Initializes the calendar object.
* * `boolean book(int start, int end)` Returns `true` if the event can be added to
*   the calendar successfully without causing a double booking. Otherwise,
*   return `false` and do not add the event to the calendar.
*
*
* Example 1:
*
* Input
* ["MyCalendar", "book", "book", "book"]
* [[], [10, 20], [15, 25], [20, 30]]
* Output
* [null, true, false, true]
* Explanation
* MyCalendar myCalendar = new MyCalendar();
* myCalendar.book(10, 20); // return True
* myCalendar.book(15, 25); // return False, It can not be booked because time 15 i
* s already booked by another event.
* myCalendar.book(20, 30); // return True, The event can be booked, as the first e
* vent takes every time less than 20, but not including 20.
*
*
* Constraints:
*
* * `0 <= start < end <= 109`
* * At most `1000` calls will be made to `book`.
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

// NOT SOLVED: thought about btree map but wasn't able to figure out how example it can be exploited
struct MyCalendar {
    intervals: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        MyCalendar { intervals: Vec::new() }
    }

    // O(n)
    fn book(&mut self, start: i32, end: i32) -> bool {
        for (s, e) in &self.intervals {
            if !((start < *s || start >= *e) && (end <= *s || end > *e)) || (start <= *s && end >= *e) {
                return false;
            }
        }
        self.intervals.push((start, end));
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
// @leetup=code

// @leetup=inject:after_code

struct Solution;

#[derive(Default)]
struct BetterCalendar {
    tree: BTreeMap<i32, i32>,
}

impl BetterCalendar {
    fn new() -> Self {
        Default::default()
    }

    // O(log(n))
    fn book(&mut self, start: i32, end: i32) -> bool {
        // log (n)
        for (&s, _) in self.tree.range(start..).take(1) {
            if s < end { return false; }
        }
        // log(n)
        for (_, &e) in self.tree.range(..start).rev().take(1) {
            if e > start { return false; }
        }
        // log(n)
        self.tree.insert(start, end);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut cal = BetterCalendar::new();
        assert!(cal.book(10, 20));
        assert!(!cal.book(10, 30));
    }
}
// @leetup=inject:after_code
