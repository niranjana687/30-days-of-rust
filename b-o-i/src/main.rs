use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<i32>().expect("invalid input");

    let mut m = String::new();
    io::stdin().read_line(&mut m).unwrap();
    let m = m.trim().parse::<i32>().expect("invalid input");

    let t_res = n - m;
    if t_res < 0 {
        println!("{}", t_res*(-1));
    } else {
        println!("{}", t_res);
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let s_len = input.len();
    
   


}