// @leetup=custom
// @leetup=info id=482 lang=rust slug=license-key-formatting

/*
* You are given a license key represented as a string `s` that consists of only
* alphanumeric characters and dashes. The string is separated into `n + 1` groups
* by `n` dashes. You are also given an integer `k`.
*
* We want to reformat the string `s` such that each group contains exactly `k`
* characters, except for the first group, which could be shorter than `k` but
* still must contain at least one character. Furthermore, there must be a dash
* inserted between two groups, and you should convert all lowercase letters to
* uppercase.
*
* Return *the reformatted license key*.
*
*
* Example 1:
*
* Input: s = "5F3Z-2e-9-w", k = 4
* Output: "5F3Z-2E9W"
* Explanation: The string s has been split into two parts, each part has 4 cha
* racters.
* Note that the two extra dashes are not needed and can be removed.
*
* Example 2:
*
* Input: s = "2-5g-3-J", k = 2
* Output: "2-5G-3J"
* Explanation: The string s has been split into three parts, each part has 2 c
* haracters except the first part as it could be shorter as mentioned above.
*
*
* Constraints:
*
* * `1 <= s.length <= 105`
* * `s` consists of English letters, digits, and dashes `'-'`.
* * `1 <= k <= 104`
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
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut reversed_result = String::with_capacity(s.len());
        let mut counter = 0;
        for c in s.chars().rev() {
            if c == '-' {
                continue;
            }
            if counter == k {
                reversed_result.push('-');
                counter = 0;
            }
            reversed_result.push(c.to_ascii_uppercase());
            counter += 1;
        }
        reversed_result.chars().rev().collect()
    }
}
// @leetup=code

// @leetup=inject:after_code

struct Solution;

// @leetup=inject:after_code
