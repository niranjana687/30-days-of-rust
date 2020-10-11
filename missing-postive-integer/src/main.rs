fn main() {
    println!("Hello, world!");
}

fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
   
    let mut num = 0;
    let mut pos = 0;

    for i in arr.iter() {
        if pos != k {
            if num != *i {
                num += 1;
                pos += 1;
            } else {
                num += 2;
                
            }
        }else {
            return num;
        }
        
    } return num;
    
}