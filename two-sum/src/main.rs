// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         for i in nums.iter() {
            
//         }
//     }
// }
use std::collections::HashMap;
fn main() {
 let v = vec![2,5,6,3,444,5,2,5,6,1];
 let target: i32 = 445;
 let res: Vec<i32> = two_sum(v, target);
 println!("{:?}", res); 
}
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        if seen.contains_key(num) {
            return vec![seen[num] as i32, i as i32]
        } else {
            seen.insert(target - num , i);
        }
    }vec![]
}