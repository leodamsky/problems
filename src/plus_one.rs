// @leetup=custom
// @leetup=info id=66 lang=rust slug=plus-one

/*
* Given a non-empty array of decimal digits representing a non-negative
* integer, increment one to the integer.
* 
* The digits are stored such that the most significant digit is at the head of the
* list, and each element in the array contains a single digit.
* 
* You may assume the integer does not contain any leading zero, except the number
* 0 itself.
* 
* 
* Example 1:
* 
* Input: digits = [1,2,3]
* Output: [1,2,4]
* Explanation: The array represents the integer 123.
* 
* Example 2:
* 
* Input: digits = [4,3,2,1]
* Output: [4,3,2,2]
* Explanation: The array represents the integer 4321.
* 
* Example 3:
* 
* Input: digits = [0]
* Output: [1]
* 
* 
* Constraints:
* 
* * `1 <= digits.length <= 100`
* * `0 <= digits[i] <= 9`
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
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for digit in digits.iter_mut().rev() {
            if *digit == 9 {
                *digit = 0;
            } else {
                *digit += 1;
                return digits;
            }
        }
        // we're here because all the digits are nines
        digits.insert(0, 1);
        digits
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
        assert_eq!(Solution::plus_one(vec![9]), [1, 0]);
    }
}

// @leetup=inject:after_code
