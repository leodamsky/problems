impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        if intervals.len() < 2 {
            return true;
        }
        intervals.sort_by_key(|i| i[0]);
        for i in 0..(intervals.len() - 1) {
            if intervals[i][1] > intervals[i + 1][0] {
                return false;
            }
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::can_attend_meetings(vec![vec![0, 30], vec![5, 10], vec![15, 20]]),
            false
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::can_attend_meetings(vec![vec![7, 10], vec![2, 4]]),
            true
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::can_attend_meetings(vec![vec![1, 5], vec![8, 9]]),
            true
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::can_attend_meetings(vec![vec![9, 10], vec![4, 9], vec![4, 17]]),
            false
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::can_attend_meetings(vec![vec![1, 5], vec![8, 10], vec![6, 17]]),
            false
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            Solution::can_attend_meetings(vec![vec![12, 13], vec![6, 11], vec![2, 19]]),
            false
        );
    }

    #[test]
    fn test7() {
        assert_eq!(
            Solution::can_attend_meetings(vec![]),
            true
        )
    }
}
