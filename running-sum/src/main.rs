fn main() {
    println!("Hello, world!");
}

fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    let mut sum = 0;
    
    for i in nums.iter() {
        sum += *i;
        res.push(sum);
    }res
}