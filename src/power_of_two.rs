// @leetup=custom
// @leetup=info id=231 lang=rust slug=power-of-two

/*
* Given an integer `n`, return *`true` if it is a power of two. Otherwise, return
* `false`*.
* 
* An integer `n` is a power of two, if there exists an integer `x` such that `n ==
* 2x`.
* 
* 
* Example 1:
* 
* Input: n = 1
* Output: true
* Explanation: 20 = 1
* 
* Example 2:
* 
* Input: n = 16
* Output: true
* Explanation: 24 = 16
* 
* Example 3:
* 
* Input: n = 3
* Output: false
* 
* Example 4:
* 
* Input: n = 4
* Output: true
* 
* Example 5:
* 
* Input: n = 5
* Output: false
* 
* 
* Constraints:
* 
* * `-231 <= n <= 231 - 1`
* 
* 
* Follow up: Could you solve it without loops/recursion?
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

/*
Ideal solution counts bits or uses XOR:
https://leetcode.com/problems/power-of-two/discuss/677042/Rust-3-solutions-explained-one-O(log(n))-and-two-O(1)-one-liners.-(0-ms-faster-than-100.)
1. n > 0 && n.count_ones() == 1
2. n > 0 && (n & (n - 1)) == 0
 */
impl Solution {
    // O(log(n))
    pub fn is_power_of_two(n: i32) -> bool {
        let mut tmp = 1;
        while tmp < n {
            if let Some(res) = tmp.checked_mul(2) {
                tmp = res;
            } else {
                break;
            }
        }
        tmp == n
    }
}
// @leetup=code

// @leetup=inject:after_code

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn prop(n: i32) {
        if n == 0 {
            assert!(!Solution::is_power_of_two(n));
        } else {
            let i = (n as f64).log2();
            let expected_res = i == i.floor();
            assert_eq!(Solution::is_power_of_two(n), expected_res);
        }
    }
}

// @leetup=inject:after_code
