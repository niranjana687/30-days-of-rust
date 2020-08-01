use std::io;

fn main() {
    let mut n = String::new();
    let mut s = String::new();

    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<i32>().expect("expected integer");

    io::stdin().read_line(&mut s).unwrap();
    
    println!("{}", n*2);
    println!("{}",s);
}