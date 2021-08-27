// @leetup=custom
// @leetup=info id=11 lang=rust slug=container-with-most-water

/*
* Given `n` non-negative integers `a1, a2, ..., an` , where each represents a
* point at coordinate `(i, ai)`. `n` vertical lines are drawn such that the two
* endpoints of the line `i` is at `(i, ai)` and `(i, 0)`. Find two lines, which,
* together with the x-axis forms a container, such that the container contains the
* most water.
*
* Notice that you may not slant the container.
*
*
* Example 1:
*
* []
* Input: height = [1,8,6,2,5,4,8,3,7]
* Output: 49
* Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,
* 8,3,7]. In this case, the max area of water (blue section) the container can con
* tain is 49.
*
* Example 2:
*
* Input: height = [1,1]
* Output: 1
*
* Example 3:
*
* Input: height = [4,3,2,1,4]
* Output: 16
*
* Example 4:
*
* Input: height = [1,2,1]
* Output: 2
*
*
* Constraints:
*
* * `n == height.length`
* * `2 <= n <= 105`
* * `0 <= height[i] <= 104`
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
    pub fn max_area(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut i = 0;
        let mut j = n - 1;
        let mut area = height[i].min(height[j]) * (j - i) as i32;

        while i != j {
            area = area.max(height[i].min(height[j]) * (j - i) as i32);
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }

        area
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
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_area(vec![2, 3, 4, 5, 18, 17, 6]), 17);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test3() {
            assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 25, 7]), 49);
        }
    }
}

// @leetup=inject:after_code
