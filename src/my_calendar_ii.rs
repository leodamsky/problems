// @leetup=custom
// @leetup=info id=731 lang=rust slug=my-calendar-ii

/*
* You are implementing a program to use as your calendar. We can add a new event
* if adding the event will not cause a triple booking.
*
* A triple booking happens when three events have some non-empty intersection
* (i.e., some moment is common to all the three events.).
*
* The event can be represented as a pair of integers `start` and `end` that
* represents a booking on the half-open interval `[start, end)`, the range of real
* numbers `x` such that `start <= x < end`.
*
* Implement the `MyCalendarTwo` class:
*
* * `MyCalendarTwo()` Initializes the calendar object.
* * `boolean book(int start, int end)` Returns `true` if the event can be added to
*   the calendar successfully without causing a triple booking. Otherwise,
*   return `false` and do not add the event to the calendar.
*
*
* Example 1:
*
* Input
* ["MyCalendarTwo", "book", "book", "book", "book", "book", "book"]
* [[], [10, 20], [50, 60], [10, 40], [5, 15], [5, 10], [25, 55]]
* Output
* [null, true, true, true, false, true, true]
* Explanation
* MyCalendarTwo myCalendarTwo = new MyCalendarTwo();
* myCalendarTwo.book(10, 20); // return True, The event can be booked.
* myCalendarTwo.book(50, 60); // return True, The event can be booked.
* myCalendarTwo.book(10, 40); // return True, The event can be double booked.
* myCalendarTwo.book(5, 15);  // return False, The event ca not be booked, because
*  it would result in a triple booking.
* myCalendarTwo.book(5, 10); // return True, The event can be booked, as it does n
* ot use time 10 which is already double booked.
* myCalendarTwo.book(25, 55); // return True, The event can be booked, as the time
*  in [25, 40) will be double booked with the third event, the time [40, 50) will
* be single booked, and the time [50, 55) will be double booked with the second ev
* ent.
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

// NOT SOLVED
use std::collections::BTreeMap;

#[derive(Default)]
struct MyCalendarTwo {
    tree: BTreeMap<i32, i32>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        Default::default()
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        *self.tree.entry(start).or_insert(0) += 1;
        *self.tree.entry(end).or_insert(0) -= 1;
        let mut active = 0;
        // this is the main idea
        // it performs an ordered run with overlapping bookings
        for &v in self.tree.values() {
            active += v;
            if active >= 3 {
                *self.tree.entry(start).or_insert(0) -= 1;
                *self.tree.entry(end).or_insert(0) += 1;
                return false;
            }
        }
        true
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */
// @leetup=code

// @leetup=inject:after_code

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn simple() {
        let mut cal = MyCalendarTwo::new();
        assert!(cal.book(10, 20));
        assert!(cal.book(50, 60));
        assert!(cal.book(10, 40));
        assert!(!cal.book(5, 15));
        assert!(cal.book(5, 10));
        assert!(cal.book(25, 55));
    }
}

// @leetup=inject:after_code
