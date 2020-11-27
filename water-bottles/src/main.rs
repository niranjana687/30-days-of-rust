fn main() {
    println!("Hello, world!");
}


fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
    let mut empty = 0;
    let mut ans = 0;
    
    while empty + num_bottles >= num_exchange {
        ans += num_bottles;
        empty += num_bottles;
        num_bottles = empty/num_exchange;
        empty = empty%num_exchange;
    } return ans + num_bottles;
}