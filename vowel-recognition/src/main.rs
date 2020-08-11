use std::io;

fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let num = num.trim().parse::<u32>().expect("invalid input");
    
    let mut user_input = String::new();
    let mut adder = String::new();

    for i in 0..num {
        user_input.as_str();
        adder = input();
        user_input = user_input.clone() + adder.as_str() ;
     
    }
    println!("{}", user_input);
    
}

fn input() -> String {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).unwrap();
    inp
}