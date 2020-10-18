fn main() {
    println!("Hello, world!");
}


fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32>   {
        
    let mut copy = nums;
        copy.sort();

        let mut result = vec![];
        let len = copy.len() as i32;

        for i in 1..=len {
            if copy.binary_search(&i).is_err() {
                result.push(i);
            }
        }result


  

}