fn main() {
    println!("Hello, world!");
}

fn reverse_string(s: &mut Vec<char>) {
    let len = s.len();
    
    for i in 0..len{
            s[i] = s[len - i - 1];
        }
}