// @leetup=custom
// @leetup=info id=43 lang=rust slug=multiply-strings

/*
* Given two non-negative integers `num1` and `num2` represented as strings, return
* the product of `num1` and `num2`, also represented as a string.
*
* Note: You must not use any built-in BigInteger library or convert the inputs
* to integer directly.
*
*
* Example 1:
*
* Input: num1 = "2", num2 = "3"
* Output: "6"
*
* Example 2:
*
* Input: num1 = "123", num2 = "456"
* Output: "56088"
*
*
* Constraints:
*
* * `1 <= num1.length, num2.length <= 200`
* * `num1` and `num2` consist of digits only.
* * Both `num1` and `num2` do not contain any leading zero, except the number `0`
*   itself.
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
    pub fn multiply(num1: String, num2: String) -> String {
        // handle annoying edge cases
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        // let's allocate less
        let mut res = Vec::with_capacity(num1.len() + num2.len());

        // process numbers in sliding windows:
        //    *  123
        //       456
        // ---------
        // +  |7|3|8
        //  |6|1|5
        // 4|9|2
        let mut iters = Vec::with_capacity(num1.len());
        for coefficient in num2.chars().rev().filter_map(|c| c.to_digit(10)) {
            iters.push(NumFromCharIterator::new(num1.chars().rev(), coefficient));
        }

        let mut lower = 0;
        let mut upper = 1;
        let mut carry = 0;
        loop {
            let mut modified = false;
            let mut sum = 0;
            for i in lower..upper {
                if let Some(digit) = iters.get_mut(i).and_then(|iter| iter.next()) {
                    modified = true;
                    sum += digit;
                    // indicates ith sliding window has exhausted itself
                } else if lower == i {
                    lower += 1;
                }
            }
            if carry != 0 {
                modified = true;
                sum += carry;
            }
            if !modified {
                break;
            }

            res.push(std::char::from_digit(sum % 10, 10).unwrap());
            carry = sum / 10;
            upper += 1;
        }

        res.reverse();
        res.into_iter().collect()
    }
}

struct NumFromCharIterator<T> {
    digits: T,
    coefficient: u32,
    carry: u32,
}

impl<T> NumFromCharIterator<T> {
    fn new(digits: T, coefficient: u32) -> NumFromCharIterator<T> {
        NumFromCharIterator {
            digits,
            coefficient,
            carry: 0,
        }
    }
}

impl<T> Iterator for NumFromCharIterator<T>
where
    T: Iterator<Item = char>,
{
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if let Some(digit) = self.digits.next() {
            let digit = digit.to_digit(10).unwrap();
            let n = digit * self.coefficient + self.carry;
            self.carry = n / 10;
            Some(n % 10)
        } else if self.carry != 0 {
            let tmp = self.carry;
            self.carry = 0;
            Some(tmp)
        } else {
            None
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
        let res = Solution::multiply("2".to_string(), "3".to_string());
        assert_eq!(res, "6".to_string())
    }

    #[test]
    fn test2() {
        let res = Solution::multiply("123".to_string(), "456".to_string());
        assert_eq!(res, "56088".to_string())
    }

    #[test]
    fn test3() {
        let res = Solution::multiply("9".to_string(), "99".to_string());
        assert_eq!(res, "891".to_string())
    }

    #[test]
    fn test4() {
        let res = Solution::multiply("123456789".to_string(), "987654321".to_string());
        assert_eq!(res, "121932631112635269".to_string())
    }

    #[test]
    fn test5() {
        let res = Solution::multiply("0".to_string(), "0".to_string());
        assert_eq!(res, "0".to_string())
    }

    #[test]
    fn test6() {
        let res = Solution::multiply("408".to_string(), "5".to_string());
        assert_eq!(res, "2040".to_string())
    }
}

// @leetup=inject:after_code
