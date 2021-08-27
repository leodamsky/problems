// @leetup=custom
// @leetup=info id=3 lang=rust slug=longest-substring-without-repeating-characters

/*
* Given a string `s`, find the length of the longest substring without repeating
* characters.
* 
* 
* Example 1:
* 
* Input: s = "abcabcbb"
* Output: 3
* Explanation: The answer is "abc", with the length of 3.
* 
* Example 2:
* 
* Input: s = "bbbbb"
* Output: 1
* Explanation: The answer is "b", with the length of 1.
* 
* Example 3:
* 
* Input: s = "pwwkew"
* Output: 3
* Explanation: The answer is "wke", with the length of 3.
* Notice that the answer must be a substring, "pwke" is a subsequence and not a su
* bstring.
* 
* Example 4:
* 
* Input: s = ""
* Output: 0
* 
* 
* Constraints:
* 
* * `0 <= s.length <= 5 * 104`
* * `s` consists of English letters, digits, symbols and spaces.
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
use std::collections::hash_map::Entry;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        let mut chars = HashMap::new();
        // we need head to detect chars that we jumped over when we met a duplicate
        let mut head = 0;
        for (tail, c) in s.chars().enumerate() {
            match chars.entry(c) {
                Entry::Occupied(e) if *e.get() >= head => {
                    head = *e.get() + 1;
                },
                _ => longest = longest.max(tail - head + 1),
            }
            chars.insert(c, tail);
        }
        longest as i32
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
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3)
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::length_of_longest_substring("tmmzuxt".to_string()), 5)
    }
}

// @leetup=inject:after_code
