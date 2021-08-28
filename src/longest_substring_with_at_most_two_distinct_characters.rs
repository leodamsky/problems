// @leetup=custom
// @leetup=info id=159 lang=rust slug=longest-substring-with-at-most-two-distinct-characters

/*
* Given a string `s`, return *the length of the longest substring that contains at
* most two distinct characters*.
*
*
* Example 1:
*
* Input: s = "eceba"
* Output: 3
* Explanation: The substring is "ece" which its length is 3.
*
* Example 2:
*
* Input: s = "ccaabbb"
* Output: 5
* Explanation: The substring is "aabbb" which its length is 5.
*
*
* Constraints:
*
* * `1 <= s.length <= 105`
* * `s` consists of English letters.
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

use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
        let mut max = 0;
        let mut head = 0;
        let mut front_char = '\0';
        let mut front_char_idx = 0;
        let mut distinct_chars = HashSet::with_capacity(2);
        for (tail, c) in s.chars().enumerate() {
            if distinct_chars.len() != 2 || distinct_chars.contains(&c) {
                distinct_chars.insert(c);
                if front_char != c {
                    front_char = c;
                    front_char_idx = tail;
                }
            } else {
                // it's fine as we have a low and constant number of chars
                distinct_chars = distinct_chars
                    .into_iter()
                    .filter(|k| *k == front_char)
                    .collect();
                distinct_chars.insert(c);
                head = front_char_idx;
                front_char = c;
                front_char_idx = tail;
            }
            max = max.max(tail - head + 1);
        }
        max as i32
    }
}
// @leetup=code

// @leetup=inject:after_code

struct Solution;

// @leetup=inject:after_code

// 1i,2,3,4,1,2i,6,2j,6
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::length_of_longest_substring_two_distinct("eceba".to_string());
        assert_eq!(res, 3);
    }

    #[test]
    fn test2() {
        let res = Solution::length_of_longest_substring_two_distinct("ccaabbb".to_string());
        assert_eq!(res, 5);
    }
}
