fn main() {
    println!("Hello, world!");
}

fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
   
    let mut missing_array = vec![];
    let last = arr.last().unwrap();
    let mut num = 1;
    
    for i in arr.iter() {
        
            while num != *i || num > *last {
                missing_array.push(num);
                num += 1;
            }
    }

    let pos = k-1;
    return missing_array[pos as usize];
    
}