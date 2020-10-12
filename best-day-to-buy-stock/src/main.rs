fn main() {
    println!("Hello, world!");
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buying_price = prices[0]; 
    let mut profit = 0; 

    for i in 1..prices.len() {
        if prices[i] < buying_price {
            buying_price = prices[i];
        } else if prices[i] -  buying_price > profit {
            profit = prices[i] -  buying_price
        }
    }profit
    

}
