fn main() {
    println!("Hello, world!");
}


fn rob(nums: Vec<i32>)  {

    let mut sum = 0;
    if nums.len() <= 2 && nums.len() >= 1{
        return *nums.iter().max().unwrap() as i32;
    } else {
        
         let mut  i = 0; 
            loop{
              if i < nums.len() {
                sum += nums[i];
                i += 2; 
              }else {
                  break;
                }
                
          }sum }
            
            
        
}

