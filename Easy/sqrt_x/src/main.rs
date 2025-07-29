/*
Hint
Given a non-negative integer x, return the square root of x rounded down to the nearest integer. 
The returned integer should be non-negative as well.

You must not use any built-in exponent function or operator.

For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.
 

Example 1:

Input: x = 4
Output: 2
Explanation: The square root of 4 is 2, so we return 2.
Example 2:

Input: x = 8
Output: 2
Explanation: The square root of 8 is 2.82842..., and since we round it down to the nearest integer, 2 is returned.


Constraints:

0 <= x <= 231 - 1
*/
struct  Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2{
            return x;
        }
        for i in 1..=x{
            if (i as i64) * (i as i64) <= x as i64 && ((i as i64)+1)*((i as i64)+1) > x as i64{
                return i;
            }
        }
        x
    }
}

fn main(){
    let result: i32 = Solution::my_sqrt(2147395600);
    println!("{}", result);

}