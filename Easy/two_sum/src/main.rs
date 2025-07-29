struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let length: usize = nums.len();
        for i in 0..length {
            for j in i + 1..length {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32]; // Retorna immediatament
                }
            }
        }

        vec![] // Si no es troba cap parella
    }
}

fn main() {
    let res = Solution::two_sum(vec![2, 7, 11, 15], 22);
    println!("{:?}", res);
}


// Evidenment jo no he fet això i em sap molt de greu, però la idea de com resoldre'l si, el que passa és que encara no sé com funciona rust...s