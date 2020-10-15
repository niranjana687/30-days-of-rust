fn main() {
    println!("Hello, world!");
}


fn max_sub_array(nums: Vec<i32>)  {
        let mut max_so_far = 0;
        let mut max_ending_here = 0;

        for i in nums.iter() {
            max_ending_here += i; 
        }
}