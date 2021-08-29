// @leetup=custom
// @leetup=info id=844 lang=rust slug=backspace-string-compare

/*
* Given two strings `s` and `t`, return `true` *if they are equal when both are
* typed into empty text editors*. `'#'` means a backspace character.
*
* Note that after backspacing an empty text, the text will continue empty.
*
*
* Example 1:
*
* Input: s = "ab#c", t = "ad#c"
* Output: true
* Explanation: Both s and t become "ac".
*
* Example 2:
*
* Input: s = "ab##", t = "c#d#"
* Output: true
* Explanation: Both s and t become "".
*
* Example 3:
*
* Input: s = "a##c", t = "#a#c"
* Output: true
* Explanation: Both s and t become "c".
*
* Example 4:
*
* Input: s = "a#c", t = "b"
* Output: false
* Explanation: s becomes "c" while t becomes "b".
*
*
* Constraints:
*
* * `1 <= s.length, t.length <= 200`
* * `s` and `t` only contain lowercase letters and `'#'` characters.
*
*
* Follow up: Can you solve it in `O(n)` time and `O(1)` space?
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
    pub fn backspace_compare(mut s: String, mut t: String) -> bool {
        fn sanitize_back(s: &mut String) {
            while let Some('#') = s.chars().next_back() {
                let mut counter = 0;
                while let Some(c) = s.pop() {
                    match c {
                        '#' => counter += 1,
                        _ => counter -= 1,
                    }
                    if counter == 0 {
                        break;
                    }
                }
            }
        }

        while !s.is_empty() || !t.is_empty() {
            sanitize_back(&mut s);
            sanitize_back(&mut t);
            let c = s.pop();
            let c2 = t.pop();
            if c != c2 {
                return false;
            }
        }
        true
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
        let res = Solution::backspace_compare("ab##".to_string(), "c#d#".to_string());
        assert!(res);
    }

    #[test]
    fn t2() {
        let res = Solution::backspace_compare("a##c".to_string(), "#a#c".to_string());
        assert!(res);
    }

    #[test]
    fn t3() {
        let res = Solution::backspace_compare("xywrrmp".to_string(), "xywrrmu#p".to_string());
        assert!(res);
    }
}

// @leetup=inject:after_code
