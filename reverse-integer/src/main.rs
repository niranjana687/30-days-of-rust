fn main() {
    let n :i32 = 4567;
    let res = reverse(n);
    println!("{}", res);
}

fn reverse(x: i32) -> i32 {
    let operator = 0;

    if x == 0 {
        return 0;
    }

    let mut num = x as i64;
    let mut param:i64 = 0;
    let mut result:i64 = 0;
    while num <= -10 || num >= 10 {
        param = num % 10;
        num = num / 10;
        result = result * 10 + param;
    }
    result = result * 10 + num;

    if result < i32::min_value() as i64 || result > i32::max_value() as i64{
        return 0;
    }
    return result as i32;
}