// @leetup=custom
// @leetup=info id=15 lang=rust slug=3sum

/*
* Given an integer array nums, return all the triplets `[nums[i], nums[j],
* nums[k]]` such that `i != j`, `i != k`, and `j != k`, and `nums[i] + nums[j] +
* nums[k] == 0`.
*
* Notice that the solution set must not contain duplicate triplets.
*
*
* Example 1:
*
* Input: nums = [-1,0,1,2,-1,-4]
* Output: [[-1,-1,2],[-1,0,1]]
*
* Example 2:
*
* Input: nums = []
* Output: []
*
* Example 3:
*
* Input: nums = [0]
* Output: []
*
*
* Constraints:
*
* * `0 <= nums.length <= 3000`
* * `-105 <= nums[i] <= 105`
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

fn two_sum_ii(nums: &[i32], i: usize, res: &mut Vec<Vec<i32>>) {
    let mut lo = i + 1;
    let mut hi = nums.len() - 1;
    while lo < hi {
        match (nums[lo] + nums[hi] + nums[i]).cmp(&0) {
            Ordering::Less => lo += 1,
            Ordering::Equal => {
                let triplet = vec![nums[i], nums[lo], nums[hi]];
                res.push(triplet);
                lo += 1;
                hi -= 1;
                // skip duplicates
                while lo < hi && nums[lo] == nums[lo - 1] {
                    lo += 1;
                }
            }
            Ordering::Greater => hi -= 1,
        }
    }
}

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        nums.sort();
        for (i, pivot) in nums.iter().copied().enumerate().take(nums.len() - 2) {
            if i == 0 || nums[i - 1] != pivot {
                two_sum_ii(&nums, i, &mut res);
            }
        }
        res
    }
}
// @leetup=code

// @leetup=inject:after_code

struct Solution;

// @leetup=inject:after_code
