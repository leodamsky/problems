// @leetup=custom
// @leetup=info id=975 lang=rust slug=odd-even-jump

/*
* You are given an integer array `arr`. From some starting index, you can make a
* series of jumps. The (1st, 3rd, 5th, ...) jumps in the series are called
* odd-numbered jumps, and the (2nd, 4th, 6th, ...) jumps in the series are
* called even-numbered jumps. Note that the jumps are numbered, not the
* indices.
*
* You may jump forward from index `i` to index `j` (with `i < j`) in the following
* way:
*
* * During odd-numbered jumps (i.e., jumps 1, 3, 5, ...), you jump to the
*   index `j` such that `arr[i] <= arr[j]` and `arr[j]` is the smallest possible
*   value. If there are multiple such indices `j`, you can only jump to the
*   smallest such index `j`.
* * During even-numbered jumps (i.e., jumps 2, 4, 6, ...), you jump to the
*   index `j` such that `arr[i] >= arr[j]` and `arr[j]` is the largest possible
*   value. If there are multiple such indices `j`, you can only jump to the
*   smallest such index `j`.
* * It may be the case that for some index `i`, there are no legal jumps.
*
* A starting index is good if, starting from that index, you can reach the end
* of the array (index `arr.length - 1`) by jumping some number of times (possibly
* 0 or more than once).
*
* Return *the number of good starting indices*.
*
*
* Example 1:
*
* Input: arr = [10,13,12,14,15]
* Output: 2
* Explanation:
* From starting index i = 0, we can make our 1st jump to i = 2 (since arr[2] is th
* e smallest among arr[1], arr[2], arr[3], arr[4] that is greater or equal to arr[
* 0]), then we cannot jump any more.
* From starting index i = 1 and i = 2, we can make our 1st jump to i = 3, then we
* cannot jump any more.
* From starting index i = 3, we can make our 1st jump to i = 4, so we have reached
*  the end.
* From starting index i = 4, we have reached the end already.
* In total, there are 2 different starting indices i = 3 and i = 4, where we can r
* each the end with some number of
* jumps.
*
* Example 2:
*
* Input: arr = [2,3,1,1,4]
* Output: 3
* Explanation:
* From starting index i = 0, we make jumps to i = 1, i = 2, i = 3:
* During our 1st jump (odd-numbered), we first jump to i = 1 because arr[1] is the
*  smallest value in [arr[1], arr[2], arr[3], arr[4]] that is greater than or equa
* l to arr[0].
* During our 2nd jump (even-numbered), we jump from i = 1 to i = 2 because arr[2]
* is the largest value in [arr[2], arr[3], arr[4]] that is less than or equal to a
* rr[1]. arr[3] is also the largest value, but 2 is a smaller index, so we can onl
* y jump to i = 2 and not i = 3
* During our 3rd jump (odd-numbered), we jump from i = 2 to i = 3 because arr[3] i
* s the smallest value in [arr[3], arr[4]] that is greater than or equal to arr[2]
* .
* We can't jump from i = 3 to i = 4, so the starting index i = 0 is not good.
* In a similar manner, we can deduce that:
* From starting index i = 1, we jump to i = 4, so we reach the end.
* From starting index i = 2, we jump to i = 3, and then we can't jump anymore.
* From starting index i = 3, we jump to i = 4, so we reach the end.
* From starting index i = 4, we are already at the end.
* In total, there are 3 different starting indices i = 1, i = 3, and i = 4, where
* we can reach the end with some
* number of jumps.
*
* Example 3:
*
* Input: arr = [5,1,3,4,2]
* Output: 3
* Explanation: We can reach the end from starting indices 1, 2, and 4.
*
*
* Constraints:
*
* * `1 <= arr.length <= 2 * 104`
* * `0 <= arr[i] < 105`
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

use std::collections::HashMap;

use JumpResult::*;

enum JumpResult {
    ReachedEnd(bool),
    ReachedCheckpoint { from: usize, odd: bool },
}

struct Jumper {
    from: usize,
    odd: bool,
}

impl Jumper {
    fn new(from: usize) -> Jumper {
        Jumper { from, odd: true }
    }

    fn jump(&mut self, candidates: &Vec<i32>, memo: &HashMap<(usize, bool), bool>) -> JumpResult {
        if let Some(result) = memo.get(&(self.from, self.odd)) {
            return ReachedEnd(*result);
        }
        if self.from == candidates.len() - 1 {
            return ReachedEnd(true);
        }

        let mut to = None;
        for to_idx in (self.from + 1)..candidates.len() {
            let source = candidates[self.from];
            let current_target = to.map(|idx| candidates[idx]);
            let maybe_target = candidates[to_idx];
            let better_target = if self.odd {
                source <= maybe_target && current_target.map(|c| c > maybe_target).unwrap_or(true)
            } else {
                source >= maybe_target && current_target.map(|c| c < maybe_target).unwrap_or(true)
            };
            if better_target {
                to = Some(to_idx);
            }
        }
        if let Some(to) = to {
            self.odd = !self.odd;
            self.from = to;
            ReachedCheckpoint {
                from: self.from,
                odd: self.odd,
            }
        } else {
            ReachedEnd(false)
        }
    }
}

impl Solution {
    pub fn odd_even_jumps(arr: Vec<i32>) -> i32 {
        let mut good_count = 0;

        let mut memo = HashMap::with_capacity(arr.len());
        for i in 0..arr.len() {
            let mut jumper = Jumper::new(i);
            let mut processed_start_points = vec![(i, true)];
            let success: bool = loop {
                match jumper.jump(&arr, &memo) {
                    ReachedEnd(success) => {
                        break success;
                    }
                    ReachedCheckpoint { from, odd } => {
                        processed_start_points.push((from, odd));
                    }
                }
            };
            for (c_idx, odd) in processed_start_points {
                memo.insert((c_idx, odd), success);
            }
            if success {
                good_count += 1
            }
        }

        good_count
    }
}
// @leetup=code

// @leetup=inject:after_code

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Solution::odd_even_jumps(vec![10, 13, 12, 14, 15]), 2);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::odd_even_jumps(vec![2, 3, 1, 1, 4]), 3);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::odd_even_jumps(vec![5, 1, 3, 4, 2]), 3);
    }
}

// @leetup=inject:after_code
