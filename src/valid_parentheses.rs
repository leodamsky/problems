// @leetup=custom
// @leetup=info id=20 lang=rust slug=valid-parentheses

/*
* Given a string `s` containing just the characters `'('`, `')'`, `'{'`, `'}'`,
* `'['` and `']'`, determine if the input string is valid.
*
* An input string is valid if:
*
* 1. Open brackets must be closed by the same type of brackets.
* 2. Open brackets must be closed in the correct order.
*
*
* Example 1:
*
* Input: s = "()"
* Output: true
*
* Example 2:
*
* Input: s = "()[]{}"
* Output: true
*
* Example 3:
*
* Input: s = "(]"
* Output: false
*
* Example 4:
*
* Input: s = "([)]"
* Output: false
*
* Example 5:
*
* Input: s = "{[]}"
* Output: true
*
*
* Constraints:
*
* * `1 <= s.length <= 104`
* * `s` consists of parentheses only `'()[]{}'`.
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
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        fn match_char(stack: &mut Vec<char>, c: char) -> bool {
            match stack.pop() {
                None => false,
                Some(c2) => c == c2,
            }
        }

        for c in s.chars() {
            match c {
                ')' => {
                    if !match_char(&mut stack, '(') {
                        return false;
                    }
                }
                ']' => {
                    if !match_char(&mut stack, '[') {
                        return false;
                    }
                }
                '}' => {
                    if !match_char(&mut stack, '{') {
                        return false;
                    }
                }
                _ => stack.push(c),
            }
        }
        stack.is_empty()
    }
}
// @leetup=code

// @leetup=inject:after_code

struct Solution;

// @leetup=inject:after_code
