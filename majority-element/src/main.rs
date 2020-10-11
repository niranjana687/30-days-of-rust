fn main() {
    println!("Hello, world!");
}


fn majority_element(nums: Vec<i32>) -> i32 {
    use std::vec::IntoIter<i32>;

        let size = nums.len();
        let mut nums = nums;
        let new = nums.sort();
        nums.into_iter().unique().collect();
        


        for i in nums.iter() {
        let mut count = 0;
        if count < new.len()/2 {
            for j in new.iter() {
                if j == i {
                    count += 1;
                } else {
                    break;
                }
            } break;
        } else {
            return count;
        }
           
        }
}