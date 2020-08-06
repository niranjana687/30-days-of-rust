use std::char::from_digit;
use std::io;
 
fn main() {
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                let trimmed = line.trim_end();
                if trimmed.len() < 1 {
                    return;
                }
                let vs = trimmed.split(" ").collect::<Vec<&str>>();
                if vs.len() < 2 {
                    return;
                }
                println!("{}", sum(vs));
            }
            _ => {
                // Done.
                return;
            }
        }
    }
}
 
fn sum(vs: Vec<&str>) -> String {
    let len1 = vs[0].len();
    let len2 = vs[1].len();
    let mut result = String::new();
 
    let mut carry = 0;
    let mut i = 0;
    loop {
        let mut sum = carry;
        let mut chars1 = vs[0].chars();
        let mut chars2 = vs[1].chars();
 
        if i < len1 && i < len2 {
            sum += char::to_digit(chars1.nth(len1 - 1 - i).unwrap(), 10).unwrap()
                + char::to_digit(chars2.nth(len2 - 1 - i).unwrap(), 10).unwrap();
        } else if i < len1 {
            sum += char::to_digit(chars1.nth(len1 - 1 - i).unwrap(), 10).unwrap();
        } else if i < len2 {
            sum += char::to_digit(chars2.nth(len2 - 1 - i).unwrap(), 10).unwrap();
        } else {
            break;
        }
        carry = sum / 10;
        match from_digit(sum % 10, 10) {
            None => {}
            Some(new_char) => {
                result.push(new_char);
                i = i + 1;
            }
        }
    }
    if carry > 0 {
        match from_digit(carry, 10) {
            None => {}
            Some(new_char) => {
                result.push(new_char);
            }
        }
    }
 
    return result.chars().rev().collect::<String>();
}
 
