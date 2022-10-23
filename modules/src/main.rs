use::modules::my_add;
use::modules::my_sub;

pub fn main() {
   let x = my_add(2,3); 
   println!("{}",x);
   let y = my_sub(2,3); 
   println!("{}",y);
    
}
