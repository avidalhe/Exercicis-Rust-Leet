/*
9. Palindrome Number

Given an integer x, return true if x is a palindrome, and false otherwise.

Example 1:

Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.
Example 2:

Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
Example 3:

Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.


Constraints:

-231 <= x <= 231 - 1

*/
struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {

        let mut result: bool = false;
        let s:String  = x.to_string();
        if s == s.chars() // Agafa l'String i el passa com un iteredor de caràcters
        .rev()// Aplica a l'iteredor anterior un revers i el retorna girat
        .collect::<String>(){ // el que fa això és agafa tot l'iterador i el retorna com a cadena (s: String)
            result = true
        }else{
            result = false
        }
        // retornem result
        result
    }
}
// En resum aquest codi el que fa és cadena -> caràctes -> en reversa -> nova cadena
fn main(){
    let result = Solution::is_palindrome(121);
    println!("{}",result);
}