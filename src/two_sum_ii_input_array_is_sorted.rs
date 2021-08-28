// @leetup=custom
// @leetup=info id=167 lang=rust slug=two-sum-ii-input-array-is-sorted

/*
* Given an array of integers `numbers` that is already *sorted in non-decreasing
* order*, find two numbers such that they add up to a specific `target` number.
*
* Return* the indices of the two numbers (1-indexed) as an integer array
* *`answer`* of size *`2`*, where *`1 <= answer[0] < answer[1] <= numbers.length`.
*
* The tests are generated such that there is exactly one solution. You may
* not use the same element twice.
*
*
* Example 1:
*
* Input: numbers = [2,7,11,15], target = 9
* Output: [1,2]
* Explanation: The sum of 2 and 7 is 9. Therefore index1 = 1, index2 = 2.
*
* Example 2:
*
* Input: numbers = [2,3,4], target = 6
* Output: [1,3]
*
* Example 3:
*
* Input: numbers = [-1,0], target = -1
* Output: [1,2]
*
*
* Constraints:
*
* * `2 <= numbers.length <= 3 * 104`
* * `-1000 <= numbers[i] <= 1000`
* * `numbers` is sorted in non-decreasing order.
* * `-1000 <= target <= 1000`
* * The tests are generated such that there is exactly one solution.
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

use std::cmp::Ordering;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;
        while i != j {
            match (numbers[i] + numbers[j]).cmp(&target) {
                Ordering::Less => i += 1,
                Ordering::Equal => return vec![(i + 1) as i32, (j + 1) as i32],
                Ordering::Greater => j -= 1,
            }
        }
        unreachable!()
    }
}
// @leetup=code

// @leetup=inject:after_code

struct Solution;

// @leetup=inject:after_code
