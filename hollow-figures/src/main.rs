fn four_side(l: u32,b: u32) {
    for i in 1..l+1 {
        for j in 1..b+1 {
            if i==1 || j<b   {
                println!("*");  
            }else {
                println!(" ");
            }         
                           
                         
        } 
        
        }println!("*"); 
    }



fn main() {
    println!("Hello, world!");
    four_side(10, 5);
}
