fn main() {
    println!("Hello, world!");
}


fn move_zeroes(nums: &mut Vec<i32>) {
    let mut last_nonzero = 0;
    let len = nums.len() ;

    for i in 0..len {
        if nums[i] != 0 {
            
            nums[last_nonzero] = nums[i];
            last_nonzero += 1;
        }   
    }

    for i in last_nonzero..len {
        nums[i] = 0;
    }
}