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

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        fn char_idx(byte: u8) -> usize {
            (byte - b'A') as usize
        }

        let mut state = [0; 128];
        // space: O(n) build char -> count mapping
        let mut target = [0; 128];
        for b in t.bytes() {
            target[char_idx(b)] += 1;
        }
        let target = target;

        let s_bytes = s.as_bytes();

        let mut best_from = 0;
        let mut best_to = 0;

        let mut from = 0;
        let mut to = 0;

        // time: O(max(m, n))
        let chars_expected = target.iter().filter(|&&ch| ch > 0).count();
        let mut chars_found = 0;

        while to < s_bytes.len() {
            let ch = char_idx(s_bytes[to]);
            if target[ch] > 0 {
                if state[ch] + 1 == target[ch] {
                    chars_found += 1;
                }
                state[ch] += 1;
            }

            if chars_found == chars_expected {
                let mut head = char_idx(s_bytes[from]);
                // move left pointer when head:
                // 1. is not counted, not in our interest
                // 2. has a higher count than needed
                while target[head] == 0 || state[head] > target[head] {
                    if state[head] > 0 {
                        state[head] -= 1;
                    }

                    from += 1;
                    head = char_idx(s_bytes[from]);
                }

                if to - from < best_to - best_from || best_to == 0 {
                    best_from = from;
                    best_to = to + 1;
                }
            }

            to += 1;
        }

        s[best_from..best_to].to_string()
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
