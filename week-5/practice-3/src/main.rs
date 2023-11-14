//Rust program to read the height of a person
// and the print if the person is tall or short
//or an average heighted person

use std::io;

fn main() {
   let mut input1 = String::new();

   println!("\n Enter your height (in centimetres): ");
   io::stdin().read_line(&mut input1).expect("Not a valid string");
   let height:f32 = input1.trim().parse().expect("Not a valid number");

   if height >= 150.0 && height <= 170.0 {
    println!("You are average height");
   }
   else if height > 170.0 && height <= 195.0{
    println!("You are tall, damn");
   }
   else if height < 150.0 && height > 100.0{
    println!("You are short, my bad, a dwarf");
   }
   else{
    println!("Abnormal height!");
   }

}
