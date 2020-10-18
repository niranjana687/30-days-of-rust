fn main() {
    println!("Hello, world!");
}


fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_so_far = 0;
        let mut max_ending_here = 0;

        for i in nums.iter() {
            max_ending_here += i;
            if max_so_far < max_ending_here {
                max_so_far = max_ending_here;
            } 

            if max_ending_here < 0 {
                max_ending_here = 0;
            }
        } max_so_far
}