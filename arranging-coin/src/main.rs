fn main() {
    println!("Hello, world!");
}

fn arrange_coins(n: i32) -> i32 {
    
    //mathemctical soluation
    // n(n+1)/2 sum of natural numbers
    
    // return ((((1 + 8 * n as i64) as f64).sqrt() - 1f64) / 2f64) as i32;

    let mut level = 0;
    let mut coin = 1;
    let mut copy = n;

    while copy >= coin {
        copy -= coin;
        coin += 1;
        level += 1;
    } level 


}
