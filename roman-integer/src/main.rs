use std::collections::HashMap;


fn main() {
    println!("Hello, world!");
    let s = String::from("IV");
    let val = roman_to_int(s);
    println!("{}", val);
}

fn roman_to_int(s: String) -> i32 {
    

    let table: HashMap<char, i32> = [
        ('M', 1000),
        ('D', 500),
        ('C', 100),
        ('L', 50),
        ('X', 10),
        ('V', 5),
        ('I', 1)
    ].iter().cloned().collect();

    let mut res = 0;
    let mut pre = 'I';

    for c in s.chars().rev() {
        if table.get(&c).unwrap() < table.get(&pre).unwrap() {
            res -=  table.get(&c).unwrap();
        }else {
            res += table.get(&c).unwrap();
        } pre = c;
    }res
    






    
}







// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000