/*
20. Valid Parentheses

Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

Open brackets must be closed by the same type of brackets.
Open brackets must be closed in the correct order.
Every close bracket has a corresponding open bracket of the same type.

Example 1:
Input: s = "()"
Output: true

Example 2:
Input: s = "()[]{}"
Output: true

Example 3:
Input: s = "(]"
Output: false

Example 4:
Input: s = "([])"
Output: true

Example 5:
Input: s = "([)]"
Output: false

Constraints:
1 <= s.length <= 104
s consists of parentheses only '()[]{}'.
*/

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // Variable que retornarà la funció
        let mut resultat: bool = false;
        // Vector per anar sabent l'ordre al qual s'estant obrint els parèntessis.
        let mut tancament: Vec<i8> = vec![];

        // Fem un bucle per a cada un dels caràcters
        for value in s.chars(){
            match value {
                '(' => {
                    tancament.push(1);
                },
                '[' => {
                    tancament.push(2);
                },
                '{' => {
                    tancament.push(3);
                },
                ')' => {
                    if !tancament.is_empty() {
                        if let Some(last) = tancament.last() {
                            if *last == 1 {
                                tancament.pop();
                            } else {
                                return false
                            }
                        }
                    } else {
                        return false;
                    }
                },
                ']' => {
                    if !tancament.is_empty() {
                        if let Some(last) = tancament.last() {
                            if *last == 2 {
                                tancament.pop();
                            } else {
                                return false;
                            }
                        }
                    } else {
                        return false;
                    }
                },
                '}' => {
                    if !tancament.is_empty() {
                        if let Some(last) = tancament.last() {
                            // Com que last és una referència a la memòria i nosaltres volem desenpecatar aquesta referència per comprar posem el *
                            if *last == 3 {
                                tancament.pop();
                            } else {
                                return false;
                            }
                        }
                    } else {
                        return false;
                    }
                }
                _ => (),
            }
        }
        // si tots els tipus són falsos vol dir que s'ha tancat correctament tots els parèntessis
        if tancament.is_empty() {
            resultat = true;
        }
        resultat
    }
}

fn main() {
    if Solution::is_valid(String::from("[[[]")) {
        println!("Resultat = True");
    }else{
        println!("Resultat = False");
    }
}
