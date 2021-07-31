// @leetup=custom
// @leetup=info id=329 lang=rust slug=longest-increasing-path-in-a-matrix

/*
* Given an `m x n` integers `matrix`, return *the length of the longest increasing
* path in *`matrix`.
* 
* From each cell, you can either move in four directions: left, right, up, or
* down. You may not move diagonally or move outside the boundary
* (i.e., wrap-around is not allowed).
* 
* 
* Example 1:
* 
* []
* Input: matrix = [[9,9,4],[6,6,8],[2,1,1]]
* Output: 4
* Explanation: The longest increasing path is `[1, 2, 6, 9]`.
* 
* Example 2:
* 
* []
* Input: matrix = [[3,4,5],[3,2,6],[2,2,1]]
* Output: 4
* Explanation: The longest increasing path is `[3, 4, 5, 6]`. Moving diagonall
* y is not allowed.
* 
* Example 3:
* 
* Input: matrix = [[1]]
* Output: 1
* 
* 
* Constraints:
* 
* * `m == matrix.length`
* * `n == matrix[i].length`
* * `1 <= m, n <= 200`
* * `0 <= matrix[i][j] <= 231 - 1`
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

use Direction::*;

type Point = (usize, usize);

impl Solution {
    // Time: O(mn)
    // Memory: O(mn)
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let matrix_size = matrix.len() * matrix.get(0).map(|m| m.len()).unwrap_or(0);
        if matrix_size == 0 {
            return 0;
        }
        let mut memo = vec![vec![0; matrix[0].len()]; matrix.len()];
        let mut longest = 1;
        for y in 0..matrix.len() {
            for x in 0..matrix[0].len() {
                longest = longest.max(longest_path((x, y), &matrix, &mut memo));
                if longest == matrix_size {
                    return longest as i32;
                }
            }
        }
        return longest as i32;

        // O(mn) in total
        fn longest_path(point: Point, matrix: &Vec<Vec<i32>>, memo: &mut Vec<Vec<usize>>) -> usize {
            let (x, y) = point;
            if memo[y][x] > 0 {
                return memo[y][x];
            }
            let longest = 1 + [Up, Down, Left, Right]
                .iter()
                .filter_map(|direction| direction.move_next(point, matrix))
                .map(|point| longest_path(point, matrix, memo))
                .max()
                .unwrap_or_default();
            memo[y][x] = longest;
            longest
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn move_next(&self, (x, y): Point, matrix: &Vec<Vec<i32>>) -> Option<Point> {
        self.next_point((x, y))
            // is in boundary
            .filter(|&(x2, y2)| x2 != matrix[0].len() && y2 != matrix.len())
            // is less than source
            .filter(|&(x2, y2)| matrix[y2][x2] < matrix[y][x])
    }

    fn next_point(&self, (x, y): Point) -> Option<Point> {
        match self {
            Up => y.checked_sub(1).map(|y| (x, y)),
            Down => y.checked_add(1).map(|y| (x, y)),
            Left => x.checked_sub(1).map(|x| (x, y)),
            Right => x.checked_add(1).map(|x| (x, y))
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
    fn test_simple() {
        let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
        let res = Solution::longest_increasing_path(matrix);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_complext() {
        let matrix = vec![
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![19, 18, 17, 16, 15, 14, 13, 12, 11, 10],
            vec![20, 21, 22, 23, 24, 25, 26, 27, 28, 29],
            vec![39, 38, 37, 36, 35, 34, 33, 32, 31, 30],
            vec![40, 41, 42, 43, 44, 45, 46, 47, 48, 49],
            vec![59, 58, 57, 56, 55, 54, 53, 52, 51, 50],
            vec![60, 61, 62, 63, 64, 65, 66, 67, 68, 69],
            vec![79, 78, 77, 76, 75, 74, 73, 72, 71, 70],
            vec![80, 81, 82, 83, 84, 85, 86, 87, 88, 89],
            vec![99, 98, 97, 96, 95, 94, 93, 92, 91, 90],
            vec![100, 101, 102, 103, 104, 105, 106, 107, 108, 109],
            vec![119, 118, 117, 116, 115, 114, 113, 112, 111, 110],
            vec![120, 121, 122, 123, 124, 125, 126, 127, 128, 129],
            vec![139, 138, 137, 136, 135, 134, 133, 132, 131, 130],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ];
        let res = Solution::longest_increasing_path(matrix);
        assert_eq!(res, 140);
    }
}

// @leetup=inject:after_code
