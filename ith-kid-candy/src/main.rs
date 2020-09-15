fn main() {
    println!("Hello, world!");
    let  a = vec![5,99,3,4100, 78];
    let res = kids_with_candies(a, 100);
    println!("{:?}", res);
    
}

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        
    let mut result: Vec<bool> = vec![];
    let highest = candies.iter().max().unwrap();
    
    for i in candies.iter() {
        if extra_candies + i >= *highest {
            result.push(true);
        } else {
            result.push(false);
        }

    } result
}