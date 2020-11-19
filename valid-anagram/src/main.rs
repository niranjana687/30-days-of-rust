fn main() {
    println!("Hello, world!");
}

fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
        }
        let mut buckets = [0; 26];
        for (i, j) in s.as_bytes().iter().zip(t.as_bytes().iter()) {
            buckets[(i - b'a') as usize] += 1;
            buckets[(j - b'a') as usize] -= 1;
        }
        buckets == [0; 26]
}