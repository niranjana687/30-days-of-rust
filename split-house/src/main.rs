use std::io;

fn main() {
    let mut n = String::new();
    let mut s = String::new();

    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u8>().expect("invalid input");

    io::stdin().read_line(&mut s).unwrap();
    let char_item = s.chars();

    if n >= 1 && n <= 20 {
        println!("YES");
        for i in char_item {
            if i == 'H' {
                print!("{}", i);   
            }else if i == '.' {
                print!("B");
            }
        }
    }else {
        println!("NO");
    }
}