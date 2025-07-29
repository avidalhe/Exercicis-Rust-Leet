use std::vec;

/*
13. Roman to Integer

Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

I can be placed before V (5) and X (10) to make 4 and 9. 
X can be placed before L (50) and C (100) to make 40 and 90. 
C can be placed before D (500) and M (1000) to make 400 and 900.
Given a roman numeral, convert it to an integer.



Example 1:

Input: s = "III"
Output: 3
Explanation: III = 3.
Example 2:

Input: s = "LVIII"
Output: 58
Explanation: L = 50, V= 5, III = 3.
Example 3:

Input: s = "MCMXCIV"
Output: 1994
Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.


Constraints:

1 <= s.length <= 15
s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
It is guaranteed that s is a valid roman numeral in the range [1, 3999].
*/
struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        //Creem la variable que retornarem
        let mut result: i32 = 0;
        //Creem un comodí per fer una gestió simple de memòria
        let mut comodi: i32 = 0;
        //Convertim l'String a un iterador de caràcters
        let cadena: std::str::Chars<'_> = s.chars();
        //Creem una variable per guardar la lectrua anterior
        let mut anterior: Option<i32> = None;
        //Vector per guardar la conversió de nombres romans a números
        let mut s_to_int: Vec<i32> = vec![];
        for valor in cadena{
            //Convertim els símbols en valors i els guardem
            match valor {
                'I' => {
                    s_to_int.push(1);
                },
                'V' => {
                    s_to_int.push(5);
                },
                'X' => {
                    s_to_int.push(10);
                },
                'L' => {
                    s_to_int.push(50);
                },
                'C' => {
                    s_to_int.push(100);
                },
                'D'=> {
                    s_to_int.push(500);
                },
                'M' => {
                    s_to_int.push(1000);
                },
                _ => (),
            }
        }
        for mut num in s_to_int{
            // si existeix un valor anterior...
            println!("{}", num);
            if let Some(val_anterior) = anterior{
                if num > val_anterior{
                    //Copiem el valor de num a comodi
                    comodi = num;
                    //Fem la resta
                    num = num-val_anterior - val_anterior;
                    //Afegim a anterior el valor copiat de comodí
                    anterior = Some(comodi);
                } else{ //Si num és més petit o igual que l'anterior
                    anterior = Some(num);
                }
                result = result + num;
            }else{ // Si encara no hi ha cap valor anterior
                anterior = Some(num);
                result = num;       
            }
        }
        result
    }
}

fn main() {
    let s: String = String::from("MCMXCIV");
    let m = Solution::roman_to_int(s);
    println!("{}",m);
}
