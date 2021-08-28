// @leetup=custom
// @leetup=info id=163 lang=rust slug=missing-ranges

/*
* You are given an inclusive range `[lower, upper]` and a sorted unique
* integer array `nums`, where all elements are in the inclusive range.
*
* A number `x` is considered missing if `x` is in the range `[lower, upper]`
* and `x` is not in `nums`.
*
* Return *the smallest sorted list of ranges that cover every missing number
* exactly*. That is, no element of `nums` is in any of the ranges, and each
* missing number is in one of the ranges.
*
* Each range `[a,b]` in the list should be output as:
*
* * `"a->b"` if `a != b`
* * `"a"` if `a == b`
*
*
* Example 1:
*
* Input: nums = [0,1,3,50,75], lower = 0, upper = 99
* Output: ["2","4->49","51->74","76->99"]
* Explanation: The ranges are:
* [2,2] --> "2"
* [4,49] --> "4->49"
* [51,74] --> "51->74"
* [76,99] --> "76->99"
*
* Example 2:
*
* Input: nums = [], lower = 1, upper = 1
* Output: ["1"]
* Explanation: The only missing range is [1,1], which becomes "1".
*
* Example 3:
*
* Input: nums = [], lower = -3, upper = -1
* Output: ["-3->-1"]
* Explanation: The only missing range is [-3,-1], which becomes "-3->-1".
*
* Example 4:
*
* Input: nums = [-1], lower = -1, upper = -1
* Output: []
* Explanation: There are no missing ranges since there are no missing numbers.
*
* Example 5:
*
* Input: nums = [-1], lower = -2, upper = -1
* Output: ["-2"]
*
*
* Constraints:
*
* * `-109 <= lower <= upper <= 109`
* * `0 <= nums.length <= 100`
* * `lower <= nums[i] <= upper`
* * All the values of `nums` are unique.
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
    pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<String> {
        let mut res = Vec::new();

        let mut try_push_res = |lower, upper| match upper - lower {
            0..=1 => {}
            2 => res.push(format!("{}", lower + 1)),
            _ => res.push(format!("{}->{}", lower + 1, upper - 1)),
        };

        // 'lower' can also be absent, so we need to step back
        let mut prev = lower - 1;
        for i in 0..nums.len() {
            let cur = nums[i];
            try_push_res(prev, cur);
            prev = cur;
        }
        // 'upper' can also be absent, so we need to step forward
        try_push_res(prev, upper + 1);

        res
    }
}
// @leetup=code

// @leetup=inject:after_code

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::find_missing_ranges(vec![0, 1, 3, 50, 75], 0, 99);
        assert_eq!(
            res,
            ["2", "4->49", "51->74", "76->99"]
                .iter()
                .cloned()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        )
    }

    #[test]
    fn test2() {
        let res = Solution::find_missing_ranges(vec![50, 75], 0, 74);
        assert_eq!(
            res,
            ["0->49", "51->74"]
                .iter()
                .cloned()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        )
    }
}

// @leetup=inject:after_code
