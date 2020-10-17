fn main() {
    println!("Hello, world!");
}

fn reverse_string(s: &mut Vec<char>) {
    let len = s.len();
    
    for i in 0..len/2{
            let end = len - i - 1;
            s.swap(i, end);
        }
}