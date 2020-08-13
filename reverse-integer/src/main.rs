fn main() {
    let n :i32 = 4567;
    let res = reverse(n);
    println!("{}", res);
}

fn reverse(x: i32) -> i32 {
   let rev :i32 = 0;
   let rem : i32= 0;
   while x != 0 {
    let rem = x % 10;
    let rev = rev*10 + rem;
    let x = x / 10;

   }

    return rev;
}