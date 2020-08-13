//find sum of two numbers
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let mut i :i64 = 0;
    for num in line.split_whitespace() {
        i += num.parse::<i64>().expect("not an integer!");
    }
    println!("{}", i);
}
