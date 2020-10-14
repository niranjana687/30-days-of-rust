fn main() {
    println!("Hello, world!");
}


fn move_zeroes(nums: &mut Vec<i32>) {
    let mut last_nonzero = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            last_nonzero += 1;
            nums[last_nonzero] = nums[i]
        }
    }
    for i in last_nonzero..=nums.len() {
        nums[i] = 0;
    }
}