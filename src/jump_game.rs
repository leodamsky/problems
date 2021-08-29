// @leetup=custom
// @leetup=info id=55 lang=rust slug=jump-game

/*
* You are given an integer array `nums`. You are initially positioned at the
* array's first index, and each element in the array represents your maximum
* jump length at that position.
*
* Return `true`* if you can reach the last index, or *`false`* otherwise*.
*
*
* Example 1:
*
* Input: nums = [2,3,1,1,4]
* Output: true
* Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
*
* Example 2:
*
* Input: nums = [3,2,1,0,4]
* Output: false
* Explanation: You will always arrive at index 3 no matter what. Its maximum j
* ump length is 0, which makes it impossible to reach the last index.
*
*
* Constraints:
*
* * `1 <= nums.length <= 104`
* * `0 <= nums[i] <= 105`
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

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut last_pos = n - 1;
        for (i, max_jump) in nums.into_iter().enumerate().rev() {
            // can reach last good position
            if i + max_jump as usize >= last_pos {
                last_pos = i;
            }
        }
        last_pos == 0
    }
}
// @leetup=code

// @leetup=inject:after_code

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tes1() {
        let res = Solution::can_jump(vec![2, 3, 1, 1, 4]);
        assert_eq!(res, true);
    }
}

// @leetup=inject:after_code
