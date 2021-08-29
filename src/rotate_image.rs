// @leetup=custom
// @leetup=info id=48 lang=rust slug=rotate-image

/*
* You are given an `n x n` 2D `matrix` representing an image, rotate the image by
* 90 degrees (clockwise).
*
* You have to rotate the image [in-place][1], which means you have to modify
* the input 2D matrix directly. DO NOT allocate another 2D matrix and do the
* rotation.
*
*
* Example 1:
*
* []
* Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
* Output: [[7,4,1],[8,5,2],[9,6,3]]
*
* Example 2:
*
* []
* Input: matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
* Output: [[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]
*
* Example 3:
*
* Input: matrix = [[1]]
* Output: [[1]]
*
* Example 4:
*
* Input: matrix = [[1,2],[3,4]]
* Output: [[3,1],[4,2]]
*
*
* Constraints:
*
* * `matrix.length == n`
* * `matrix[i].length == n`
* * `1 <= n <= 20`
* * `-1000 <= matrix[i][j] <= 1000`
*
* [1] https://en.wikipedia.org/wiki/In-place_algorithm
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
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        fn rotate_4(matrix: &mut [&mut [i32]], times: usize) {
            let n = matrix.len();
            if n <= 1 {
                return;
            }
            let mut tmp;

            // cell ids
            // 0...1
            // .....
            // 3...2

            for i in 0..times {
                // 1. swap 0 and 1
                tmp = matrix[0][i];
                matrix[0][i] = matrix[i][n - 1];
                matrix[i][n - 1] = tmp;

                // 2. swap 0 and 2
                tmp = matrix[0][i];
                matrix[0][i] = matrix[n - 1][n - 1 - i];
                matrix[n - 1][n - 1 - i] = tmp;

                // 3. swap 0 and 3
                tmp = matrix[0][i];
                matrix[0][i] = matrix[n - 1 - i][0];
                matrix[n - 1 - i][0] = tmp;
            }
        }

        let n = matrix.len();
        for degree in 0..(n / 2) {
            let frame_size = n - degree * 2;
            // FIXME: extra allocations
            let mut rows = Vec::with_capacity(frame_size);
            for row in matrix.iter_mut().skip(degree).take(frame_size) {
                rows.push(&mut row[degree..(n - degree)]);
            }
            let times = frame_size.checked_sub(1).unwrap_or(0);
            rotate_4(&mut rows, times);
        }
    }
}
// @leetup=code

// @leetup=inject:after_code

struct Solution;

// @leetup=inject:after_code
