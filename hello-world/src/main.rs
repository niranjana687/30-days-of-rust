fn main() {
    let mut line = String::new();
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello, world!");
    println!("{}", line);
}
