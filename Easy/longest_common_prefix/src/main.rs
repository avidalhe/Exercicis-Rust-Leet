/*
14. Longest Common Prefix

Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string "".

Example 1:

Input: strs = ["flower","flow","flight"]
Output: "fl"
Example 2:

Input: strs = ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.

Constraints:

1 <= strs.length <= 200
0 <= strs[i].length <= 200
strs[i] consists of only lowercase English letters if it is non-empty.
*/

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        //Comprovem que la longitud del vector sigui més gran que 1, si ho és, només retornem el contingut d'aquest vector
        if strs.len() == 1{
            // Si la longitud del vector només és 1, retornem el primer prefix...
            let valor: Option<char> = strs[0].chars().nth(0);
            if let Some(value) = valor{
                return String::from(value);
            }
            else{
                return String::from("");
            }
        }
        //Creem la variable que guardarà el prefix
        let mut prefix: String = String::new();

        //Creem la variable que guardarà el prefix més llarg
        let mut longest_prefix: String = String::new();

        //Creem la variable per guardar la longitud més llarga
        let mut longitud: usize = 100;

        //Copiem la primera paraula del vector a un nou espai de memòria al heap, per així no tocar la principal.
        let paraula_1:String = strs[0].clone();

        //Creem una variable per guardar la priemra lletra
        let mut lletra: Option<char> = paraula_1.chars().nth(0);

        //Iterem per a cada element i busquem la longitud més petita, fem que la i només sigui una referència als elements de la cadena.
        for i in strs.iter(){
            //Comparem la longitud actual l'anterior, si l'actual és més petita, guardem el resultat.
            longitud = if i.len() < longitud {i.len()} else {longitud}
        }
        //Generem una variable de control del bucle
        let mut j:usize = 0;

        //Creem una variable per controlar si les tres lletres són iguals, és a dir, que s'ha passat la comparació.
        let mut pass: bool = false;

        // Fem un bucle mentre per seguretata
        while j < longitud{
            //Si hi ha alguna lletra a lletra. 
            if let Some(val_lletra) = lletra{
                // Iterem per a cada String i obtenim una referència de cada paraula, aquesta referència és inmutable.
                for i in 1..strs.len(){
                    // Comparem les lletres de cada paraula, si hi ha alguna que no coincideix trenquem el prefix
                    if Some(val_lletra) == strs[i].chars().nth(j){
                        // Marquem com que ha passat correctament la comparació
                        pass = true;
                        //Continuem amb l'iteració
                        continue;
                    }else{
                        // Si trobem que la primera lletra no coincideix, ja no hi ha prefix
                        if j == 0{
                            return String::new();
                        }
                        //Marquem que no ha passat la comprovació
                        pass = false;
                        // Si hi ha alguna lletra que no és igual, l'eliminem
                        break;
                    }
                }
                //Mirem si pass és true, si no ho és comparem el prefix anterior amb l'actual i si l'actual és més gran l'assignem a l'altre variable i l'eliminem.
                if pass{
                    prefix.push(val_lletra);
                }else {
                    // Només fem això en el cas de que prefix no estigui buit...
                    if longest_prefix.len() < prefix.len() && !prefix.is_empty(){
                        // Replacem les paraules anteriores per les noves, el que jo em preguntu és si afegira les que no coincideixen...
                        longest_prefix = longest_prefix.replace(longest_prefix.as_str(),prefix.clone().as_str());
                        prefix.clear();
                    }
                }
            }
            //Augmentem en 1 el valor de l'Iteració
            j += 1;
            // Mirem la següent paraula.
            lletra = paraula_1.chars().nth(j);
        }
        
        //Returnem el resultat
        if longest_prefix.is_empty() && j < 2 || j == prefix.len(){
            return prefix;
        }else {
            return longest_prefix;
        }
    }
}


fn main() {
    let strigns = vec!["ab","a"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();
    let result = Solution::longest_common_prefix(strigns);
    println!("{}", result);
}
