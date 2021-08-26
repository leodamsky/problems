// @leetup=custom
// @leetup=info id=904 lang=rust slug=fruit-into-baskets

/*
* You are visiting a farm that has a single row of fruit trees arranged from left
* to right. The trees are represented by an integer array `fruits` where
* `fruits[i]` is the type of fruit the `ith` tree produces.
*
* You want to collect as much fruit as possible. However, the owner has some
* strict rules that you must follow:
*
* * You only have two baskets, and each basket can only hold a single type
*   of fruit. There is no limit on the amount of fruit each basket can hold.
* * Starting from any tree of your choice, you must pick exactly one fruit
*   from every tree (including the start tree) while moving to the right. The
*   picked fruits must fit in one of your baskets.
* * Once you reach a tree with fruit that cannot fit in your baskets, you must
*   stop.
*
* Given the integer array `fruits`, return *the maximum number of fruits you
* can pick*.
*
*
* Example 1:
*
* Input: fruits = [1,2,1]
* Output: 3
* Explanation: We can pick from all 3 trees.
*
* Example 2:
*
* Input: fruits = [0,1,2,2]
* Output: 3
* Explanation: We can pick from trees [1,2,2].
* If we had started at the first tree, we would only pick from trees [0,1].
*
* Example 3:
*
* Input: fruits = [1,2,3,2,2]
* Output: 4
* Explanation: We can pick from trees [2,3,2,2].
* If we had started at the first tree, we would only pick from trees [1,2].
*
* Example 4:
*
* Input: fruits = [3,3,3,1,2,1,1,2,3,3,4]
* Output: 5
* Explanation: We can pick from trees [1,2,1,1,2].
*
*
* Constraints:
*
* * `1 <= fruits.length <= 105`
* * `0 <= fruits[i] < fruits.length`
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
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        // track last two fruits seen
        let mut last_fruit = -1;
        let mut second_last_fruit = -1;
        let mut last_fruit_count = 0;
        let mut curr_max = 0;
        let mut max = 0;

        for fruit in fruits {
            if fruit == last_fruit || fruit == second_last_fruit {
                curr_max += 1;
            } else {
                curr_max = last_fruit_count + 1; // last fruit + new fruit
            }

            if fruit == last_fruit {
                last_fruit_count += 1;
            } else {
                last_fruit_count = 1;
            }

            if fruit != last_fruit {
                second_last_fruit = last_fruit;
                last_fruit = fruit;
            }

            max = max.max(curr_max);
        }

        max
    }
}
// @leetup=code

// @leetup=inject:after_code

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = Solution::total_fruit(vec![0, 1, 2, 2]);
        assert_eq!(result, 3);
    }
}

// @leetup=inject:after_code
