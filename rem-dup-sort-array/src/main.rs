fn main() {
    println!("Hello, world!");
}


fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut len = nums.len();
    if len == 0 {
        return 0;
    }
    else {
        nums.dedup();
        return nums.len() as i32;
    }
    // let mut count = 0;
    // for i in 0..len {

    //     for j in i..len {
    //         if nums[i] == nums[j] {
    //             count += 1;
    //             let nums[j] = nums[j+1];
    //         } 
    //     }
        
    // } len -= count;
    // len as i32
     
}