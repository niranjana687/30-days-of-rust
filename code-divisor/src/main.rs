use std::io;


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string();
    let mut answer :i64 = 0;
    let nums = input.split(' ').collect::<Vec<&str>>();
    let mut l :i32 = nums[0].parse().unwrap();
    let mut r :i32 = nums[1].parse().unwrap();
    let mut k :i32 = nums[2].parse().unwrap();
    
    loop {
        if l > r {
            break;
        }
 
        if (l % k) == 0 {
            answer = answer + 1;
        }
 
        l = l + 1;
    }
 
    println!("{}", answer);
    }

    

    




