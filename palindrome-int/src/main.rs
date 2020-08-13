fn main() {
    let num = 676;
    println!("{}", is_palindrome(num));
    

fn is_palindrome(x: i32) -> bool {
    // let mut num = x as i64;
    // let mut param:i64 = 0;
    // let mut result:i64 = 0;
    // while num <= -10 || num >= 10 {
    //     param = num % 10;
    //     num = num / 10;
    //     result = result * 10 + param;
    // }
    // result = result * 10 + num;
    // if result as i32 == x {
    //     true
    // } else {
    //     false
    // }
    x.to_string() == x.to_string().chars().rev().collect::<String>()

    }
