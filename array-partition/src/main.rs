
fn main() {
    let v = vec![2,4,13,5,3,5,45,4,3,2,6,31,3,5,214,566,322];
    let res = array_pair_sum(v);
    println!("{}",res);
}

fn array_pair_sum(nums: Vec<i32>) -> i32 {
    let mut nums_v = nums;
    nums_v.sort_unstable();
    nums_v.iter().step_by(2).sum()
    
}