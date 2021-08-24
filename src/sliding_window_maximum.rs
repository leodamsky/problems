// @leetup=custom
// @leetup=info id=239 lang=rust slug=sliding-window-maximum

/*
* You are given an array of integers `nums`, there is a sliding window of size `k`
* which is moving from the very left of the array to the very right. You can only
* see the `k` numbers in the window. Each time the sliding window moves right by
* one position.
*
* Return *the max sliding window*.
*
*
* Example 1:
*
* Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
* Output: [3,3,5,5,6,7]
* Explanation:
* Window position                Max
* ---------------               -----
* [1  3  -1] -3  5  3  6  7       3
*  1 [3  -1  -3] 5  3  6  7       3
*  1  3 [-1  -3  5] 3  6  7       5
*  1  3  -1 [-3  5  3] 6  7       5
*  1  3  -1  -3 [5  3  6] 7       6
*  1  3  -1  -3  5 [3  6  7]      7
*
* Example 2:
*
* Input: nums = [1], k = 1
* Output: [1]
*
* Example 3:
*
* Input: nums = [1,-1], k = 1
* Output: [1,-1]
*
* Example 4:
*
* Input: nums = [9,11], k = 2
* Output: [11]
*
* Example 5:
*
* Input: nums = [4,-2], k = 2
* Output: [4]
*
*
* Constraints:
*
* * `1 <= nums.length <= 105`
* * `-104 <= nums[i] <= 104`
* * `1 <= k <= nums.length`
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

use crate::data_structures::monotonic_queue::MonotonicQueue;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() * k as usize == 0 {
            return Vec::new();
        }

        let mut queue = MonotonicQueue::new();
        let is_less = |i1: &usize, i2: &usize| nums[*i1].lt(&nums[*i2]);
        for i in 0..k {
            queue.push_by(i as usize, is_less);
        }

        let mut result = Vec::with_capacity(nums.len());
        // unwrap() is safe as we previously handled an empty input
        result.push(nums[*queue.peek().unwrap()]);
        for i in (k as usize)..nums.len() {
            if *queue.peek().unwrap() == i - k as usize {
                queue.pop();
            }
            queue.push_by(i as usize, is_less);
            result.push(nums[*queue.peek().unwrap()]);
        }
        result
    }
}
// @leetup=code

// @leetup=inject:after_code

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = Solution::max_sliding_window(vec![1, -1], 1);
        assert_eq!(result, [1, -1]);
    }

    #[test]
    fn test2() {
        let result = Solution::max_sliding_window(vec![7, 2, 4], 2);
        assert_eq!(result, [7, 4]);
    }
}

// @leetup=inject:after_code
