fn main() {
    let vec = vec![1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,0];
    let res = is_one_bit(vec);
    println!("{}", res);
}

fn is_one_bit(bits :Vec<i32>) -> bool {
    let l  = bits.len() ;
    let len  = l-2 ;
            
            if bits[len as usize] == 0 {
                return true;
            }else {
                return false;
            }
        }
        // let mut i = 0;
        // while i < bits.len() - 1 {
        //     i += bits[i] as usize + 1;
        // }
        // i == bits.len() - 1