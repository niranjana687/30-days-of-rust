fn main() {
    println!("Hello, world!");
}

fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
   
    let mut array = vec![];
    let mut count = 1;

    for i in arr.iter() {
         
        if *i != count {
            array.push(count);
            count += 1;
        } count += 1;
    }
    let k = k - 1;
    return array[k as usize];
}