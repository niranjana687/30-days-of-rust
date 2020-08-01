use std::io;


fn main() {
    let a = 45;
    let b = 45.6;
    let c = "Hello";

    let mut i = String::new();
    let mut d = String::new();
    let mut s = String::new();

    io::stdin().read_line(&mut i).unwrap();
    let i = i.trim().parse::<i32>().expect("invalid input");

    io::stdin().read_line(&mut d).unwrap();
    let d = d.trim().parse::<f64>().expect("invalid input");

    io::stdin().read_line(&mut s).unwrap();
    
    println!("Sum: {}", a + i);
    println!("D Sum: {}", b + d);

    // c.to_string();
    // c.push_str(&s.as_str());
    println!("String: {} {}",c, s);

}

