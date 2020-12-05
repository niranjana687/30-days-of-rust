use std::io;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).unwrap();
    let mut array = vec![];
    
    for num in line.split_whitespace() {
         array.push(num.parse::<i64>().expect("not an integer!"));
    }
    println!("{}", i);
}
