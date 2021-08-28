use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    // time: O(n log(n))
    // space: O(1) because we deallocate input as we transform it
    pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }

        intervals.sort_by_key(|i| i[0]);

        let mut free_rooms = BinaryHeap::with_capacity(intervals.len());
        free_rooms.push(Reverse(intervals[0][1]));

        for i in intervals[1..].iter() {
            // key part - here we free rooms lazily
            if free_rooms.peek().unwrap().0 <= i[0] {
                free_rooms.pop();
            }
            free_rooms.push(Reverse(i[1]));
        }

        return free_rooms.len() as i32;
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::min_meeting_rooms(vec![vec![0, 30], vec![5, 10], vec![15, 20]]);
        assert_eq!(res, 2);
    }

    #[test]
    fn test2() {
        let res = Solution::min_meeting_rooms(vec![vec![13, 15], vec![1, 13]]);
        assert_eq!(res, 1);
    }

    #[test]
    fn test3() {
        let res = Solution::min_meeting_rooms(vec![
            vec![0, 5],
            vec![0, 5],
            vec![0, 5],
            vec![0, 5],
            vec![6, 7],
            vec![8, 9],
            vec![9, 10],
            vec![11, 12],
        ]);
        assert_eq!(res, 4);
    }
}
