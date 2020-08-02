use std::io;

fn main() {
   let mut n = String::new();

   io::stdin().read_line(&mut n).unwrap();
   let n = n.trim().parse::<u8>().expect("invalid input");

   let c = n; 

   let mut p = 0;
   let mut m = 0;

   for i in 1..=n {
        p += i;
        m += i*2;
        if p + m < n && c < i {
            println!("Patlu");
            break;
        } else  {
            println!("Motu"); break;
        }

   }
   
       
   
}
