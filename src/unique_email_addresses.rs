// @leetup=custom
// @leetup=info id=929 lang=rust slug=unique-email-addresses

/*
* Every valid email consists of a local name and a domain name,
* separated by the `'@'` sign. Besides lowercase letters, the email may contain
* one or more `'.'` or `'+'`.
*
* * For example, in `"alice@leetcode.com"`, `"alice"` is the local name, and
*   `"leetcode.com"` is the domain name.
*
* If you add periods `'.'` between some characters in the local name part of
* an email address, mail sent there will be forwarded to the same address without
* dots in the local name. Note that this rule does not apply to domain
* names.
*
* * For example, `"alice.z@leetcode.com"` and `"alicez@leetcode.com"` forward to
*   the same email address.
*
* If you add a plus `'+'` in the local name, everything after the first plus
* sign will be ignored. This allows certain emails to be filtered. Note that
* this rule does not apply to domain names.
*
* * For example, `"m.y+name@email.com"` will be forwarded to `"my@email.com"`.
*
* It is possible to use both of these rules at the same time.
*
* Given an array of strings `emails` where we send one email to each `email[i]`,
* return *the number of different addresses that actually receive mails*.
*
*
* Example 1:
*
* Input: emails = ["test.email+alex@leetcode.com","test.e.mail+bob.cathy@leetc
* ode.com","testemail+david@lee.tcode.com"]
* Output: 2
* Explanation: "testemail@leetcode.com" and "testemail@lee.tcode.com" actually
*  receive mails.
*
* Example 2:
*
* Input: emails = ["a@leetcode.com","b@leetcode.com","c@leetcode.com"]
* Output: 3
*
*
* Constraints:
*
* * `1 <= emails.length <= 100`
* * `1 <= emails[i].length <= 100`
* * `email[i]` consist of lowercase English letters, `'+'`, `'.'` and `'@'`.
* * Each `emails[i]` contains exactly one `'@'` character.
* * All local and domain names are non-empty.
* * Local names do not start with a `'+'` character.
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

fn sanitize(email: &String) -> String {
    let mut res = String::new();
    let mut chars = email.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '@' => {
                res.push(c);
                res.extend(chars);
                break;
            },
            '.' => continue,
            '+' => {
                while let Some(c) = chars.peek() {
                    if *c == '@' {
                        break;
                    }
                    chars.next();
                }
            },
            c => res.push(c),
        }
    }
    res
}

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut unique_emails = HashSet::new();
        for email in emails {
            let sanitized = sanitize(&email);
            unique_emails.insert(sanitized);
        }
        unique_emails.len() as i32
    }
}
// @leetup=code

// @leetup=inject:after_code

struct Solution;

// @leetup=inject:after_code
