impl Solution {
    // time: O(n log(n))
    // space: O(1) because we deallocate input as we transform it
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals: Vec<_> = intervals
            .into_iter()
            .flat_map(|interval| vec![(interval[0], false), (interval[1], true)])
            .collect();
        intervals.sort_by(|i1, i2| {
            // if intervals are adjacent, ensure the 1st one ends before the 2nd one starts
            if i1.0 == i2.0 {
                i1.1.cmp(&i2.1).reverse()
            } else {
                i1.cmp(i2)
            }
        });

        let mut count = 0;
        let mut max = 0;
        for (_, end) in intervals {
            if end {
                count -= 1;
            } else {
                count += 1;
                max = max.max(count);
            }
        }
        max
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
}
