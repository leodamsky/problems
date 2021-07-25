// @leetup=custom
// @leetup=info id=1 lang=rust slug=two-sum

/*
* Given an array of integers `nums` and an integer `target`, return *indices of
* the two numbers such that they add up to `target`*.
* 
* You may assume that each input would have *exactly* one solution, and you
* may not use the *same* element twice.
* 
* You can return the answer in any order.
* 
* 
* Example 1:
* 
* Input: nums = [2,7,11,15], target = 9
* Output: [0,1]
* Output: Because nums[0] + nums[1] == 9, we return [0, 1].
* 
* Example 2:
* 
* Input: nums = [3,2,4], target = 6
* Output: [1,2]
* 
* Example 3:
* 
* Input: nums = [3,3], target = 6
* Output: [0,1]
* 
* 
* Constraints:
* 
* * `2 <= nums.length <= 104`
* * `-109 <= nums[i] <= 109`
* * `-109 <= target <= 109`
* * Only one valid answer exists.
* 
* 
* Follow-up: Can you come up with an algorithm that is less than `O(n2) `time
* complexity?
* 
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// Test comment
// Test code
// @leetup=inject:before_code_ex

// @leetup=code

// @leetup=inject:before_code
use std::collections::HashMap;

// @leetup=inject:before_code

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut memo = HashMap::new();
        for (i, n) in nums.into_iter().enumerate() {
            if let Some(k) = memo.get(&(target - n)) {
                return vec![i as i32, *k as i32];
            } else {
                memo.insert(n, i);
            }
        }
        unreachable!()
    }
}
// @leetup=code

// @leetup=inject:after_code

struct Solution; 

// @leetup=inject:after_code
