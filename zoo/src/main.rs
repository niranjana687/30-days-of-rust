use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut count_x = 0;
    let mut count_y = 0;
    let var_name = input.chars();
    for i in var_name {ßßß
        
        if i == 'z' {
            count_x += 1; 
        } else if i == 'o'{
            count_y += 1; 
        }
    }
    if count_x * 2 == count_y {
        println!("Yes");
    } else {
        println!("No");
    }
}