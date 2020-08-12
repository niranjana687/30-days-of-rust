
use std::io;




fn main() {
  let l = input();
  let s = input();

  let result = gcd(l,s);
  println!("GCD: {}", result);
}

fn gcd( s: i32, l: i32) -> i32 {
    if s == 0 {
        return l;
    } else {
        gcd(l%s, s)
    }
}  

fn input() -> i32{
    
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let num = num.trim().parse::<i32>().expect("invalid input");
    num
}