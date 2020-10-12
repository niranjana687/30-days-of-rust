fn main() {
    println!("Hello, world!");
}

fn climb_stairs(n: i32) -> i32 {
        //fibonacii number 
        if n == 1 {

            return 1;

        } else {
            let mut first = 1;
            let mut second = 2;

            for i in 3..=n {
                let mut third = first + second;
                first = second;
                second = third;
            }second
            
        }
        
}