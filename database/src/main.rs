use std::io;

fn main() {
    let mut t = String::new();
    
    io::stdin().read_line(&mut t).unwrap();
    let t = t.trim().parse::<u32>().
}
