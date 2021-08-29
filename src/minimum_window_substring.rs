// @leetup=custom
// @leetup=info id=76 lang=rust slug=minimum-window-substring

/*
* Given two strings `s` and `t` of lengths `m` and `n` respectively, return *the
* minimum window substring of *`s`* such that every character in *`t`*
* (including duplicates) is included in the window. If there is no such
* substring, return the empty string *`""`*.*
*
* The testcases will be generated such that the answer is unique.
*
* A substring is a contiguous sequence of characters within the string.
*
*
* Example 1:
*
* Input: s = "ADOBECODEBANC", t = "ABC"
* Output: "BANC"
* Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C'
* from string t.
*
* Example 2:
*
* Input: s = "a", t = "a"
* Output: "a"
* Explanation: The entire string s is the minimum window.
*
* Example 3:
*
* Input: s = "a", t = "aa"
* Output: ""
* Explanation: Both 'a's from t must be included in the window.
* Since the largest window of s only has one 'a', return empty string.
*
*
* Constraints:
*
* * `m == s.length`
* * `n == t.length`
* * `1 <= m, n <= 105`
* * `s` and `t` consist of uppercase and lowercase English letters.
*
*
* Follow up: Could you find an algorithm that runs in `O(m + n)` time?
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

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.len() > s.len() {
            return "".to_string();
        }

        let mut met = t.len();
        let mut i = 0;

        // space: O(n) build char -> count mapping
        let mut chars = HashMap::new();
        // time: O(n)
        for c in t.chars() {
            *chars.entry(c).or_insert(0) += 1;
        }
        // space: O(m) if t == n and consists of a the same character
        let mut heads: HashMap<char, VecDeque<usize>> = HashMap::new();

        let mut min = s.as_str();
        // time: O(m + n?)
        for j in 0..s.len() {
            let c = s[j..].chars().next().unwrap();
            if let Some(count) = chars.get_mut(&c) {
                heads.entry(c).or_insert(VecDeque::new()).push_back(j);
                if *count == 0 {
                    heads.get_mut(&c).unwrap().pop_front();
                }
                if *count > 0 {
                    *count -= 1;
                    met -= 1;
                }
                // guess it's O(n) in total as:
                // higher diversity of characters makes this operations cheaper
                // lower diversity of characters makes this operations rare
                if *count == 0 {
                    i = *heads.values().filter_map(|l| l.get(0)).min().unwrap_or(&j);
                }
            }
            if met == 0 && min.len() > j - i + 1 {
                min = &s[i..=j];
            }
        }

        if met == 0 {
            min.to_string()
        } else {
            "".to_string()
        }
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
        let res = Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string());
        assert_eq!(res, "BANC");
    }

    #[test]
    fn t2() {
        let res = Solution::min_window("ab".to_string(), "b".to_string());
        assert_eq!(res, "b");
    }
}

// @leetup=inject:after_code
