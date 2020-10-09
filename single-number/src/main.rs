fn main() {
    println!("Hello, world!");
}


fn single_number(nums: Vec<i32>) -> i32  {
    let mut flag = 0;
    let mut repeat_array = vec![];

    if nums.len() == 1 {
        return nums[0];
    }
    else {
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if nums[i] == nums[j] {
                    repeat_array.push(nums[i]);
                }
            }
        
        }
    }
    
}