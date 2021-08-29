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
        let mut stack = vec![0];
        let n = nums.len();
        let mut explored = vec![false; n];
        explored[0] = true;
        while let Some(i) = stack.pop() {
            if i == n - 1 {
                return true;
            }
            let max_jump = nums[i];
            for next_jump in (i + 1)..n.min(i + 1 + max_jump as usize) {
                if !explored[next_jump] {
                    explored[next_jump] = true;
                    stack.push(next_jump);
                }
            }
        }
        false
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
