// @leetup=custom
// @leetup=info id=31 lang=rust slug=next-permutation

/*
* Implement next permutation, which rearranges numbers into the
* lexicographically next greater permutation of numbers.
*
* If such an arrangement is not possible, it must rearrange it as the lowest
* possible order (i.e., sorted in ascending order).
*
* The replacement must be [in place][1] and use only constant extra memory.
*
*
* Example 1:
*
* Input: nums = [1,2,3]
* Output: [1,3,2]
*
* Example 2:
*
* Input: nums = [3,2,1]
* Output: [1,2,3]
*
* Example 3:
*
* Input: nums = [1,1,5]
* Output: [1,5,1]
*
* Example 4:
*
* Input: nums = [1]
* Output: [1]
*
*
* Constraints:
*
* * `1 <= nums.length <= 100`
* * `0 <= nums[i] <= 100`
*
* [1] http://en.wikipedia.org/wiki/In-place_algorithm
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
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        let (mut i, mut j) = (n - 1, n - 1);

        // get the index of last increasing subseq elem from right
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }

        // elems from (i..len - 1) are in descending
        if i > 0 {
            // get index of the first element >= nums[i-1]
            // equivalent to sorting the element in ascending and get the index of element right after nums[i-1]
            while j >= i && nums[i - 1] >= nums[j] {
                j -= 1;
            }
            // swap the smallest and next greater element
            nums.swap(i - 1, j);
        }

        // reverse the elements from (i to len - 1) to convert to ascending
        nums[i..n].reverse();
    }
}

// @leetup=code

// @leetup=inject:after_code

struct Solution;

// @leetup=inject:after_code
