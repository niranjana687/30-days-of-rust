use std::io;
use std::str::FromStr;
fn main() {
    let mut num = String::new();

    io::stdin().read_line(&mut num).unwrap();
    let num = num.trim().parse::<i32>().expect("invalid input");

    let i = read_values::<i64>().unwrap();
    println!("{}", num);
    
    for a in i.iter() {
        print!("{} ", a);
    }    

}

fn read_values<T :FromStr>() -> Result<Vec<T>, T::Err> {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("could not read");
    s.trim()
        .split_whitespace()
        .map(|word| word.parse())
        .collect()
}
