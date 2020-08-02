use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let dir = input.chars();
    
    

     let (pos_x , pos_y) = char_count(dir);

    println!("{} {}", pos_x, pos_y);
    

   

    

}

fn char_count(s : std::str::Chars) -> (i32, i32) {
    let mut r = 0;
    let mut l = 0;
    let mut u = 0;
    let mut d = 0;
    let mut pos_x = 0;
    let mut pos_y = 0;

    for i in s {
        if i == 'L' {
            l += 1
        } else if i == 'R' {
            r += 1
        } else if i == 'U' {
            u += 1
        }else if i == 'D' {
            d += 1
        } 
    } let pos_x = r - l;
    let pos_y = u - d;

    

    return (pos_x, pos_y) ;
   
}