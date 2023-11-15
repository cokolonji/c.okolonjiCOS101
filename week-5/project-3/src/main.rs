use std::io;

fn main() {
    let p = "Poundo Yam/Edinkainko Soup";
    let f = "Fried rice and Chicken";
    let a = "Amala and Egusi soup";
    let e = "Eba and Egusi soup";
    let w = "White Rice and Stew";

    loop{ 
    println!("What would you like to eat: \n
                      Menu                       Price \n
              P = Poundo Yam/Edinkaiko Soup   - N3,200\n
              F = Fried Rice & Chicken        - N3,000\n
              A = Amala & Ewedu Soup          - N2,500\n
              E = Eba & Egusi Soup            - N2,000\n
              W = White Rice & Stew           - N2,500\n
             ");

   
    let mut food = String::new();
    println!("Enter here:");
    io::stdin().read_line(&mut food).expect("Failed to read input");
    let food = food.trim().to_lowercase();

    if food == "p" || food == "Poundo Yam/Edinkainko Soup" {
      println!("how many portions do you desire :) {}", p);
    }
    else if food == "f" || food == "Fried rice and Chicken" {
      println!("how many portions do you desire :) {}", f);
    }
    else if food == "a" || food == "Amala and Egusi soup" {
      println!("how many portions do you desire :) {}", a);
    }
    else if food == "e" || food == "Eba and Egusi soup" {
      println!("how many portions do you desire :) {}", e);
    }
    else if food == "w" || food == "White Rice and Stew" {
      println!("how many portions do you desire :) {}", w);
    }
    else {
      println!("Not an item on the menu, Shine your eye!");
    }


    let mut portion_size = String::new();
    io::stdin().read_line(&mut portion_size).expect("Failed to read input");
    let portion_size = portion_size.trim().parse().expect("Not a valid number");



    if food == "p" || food == "Poundo Yam/Edinkainko Soup" {
      let calc:f32 = 3200.0 * portion_size;
      if calc > 10000.0 {
        let calc:f32 = calc * 0.95;
        println!("Yayyyy, you get a discount. Your total cost is N{}", calc);break;
      }
    
      else {println!("Your total price is {}", calc);break;}}

    else if food == "f" || food == "Fried rice and Chicken" {
      let calc:f32 = 3000.0 * portion_size;
      if calc > 10000.0 {
        Let calc:f32 = calc * 0.95;
        println!("Yayyyy, you get a discount. Your total cost is N{}", calc);break;
      }
    
      else {println!("Your total price is {}", calc);break;}
    
    else if food == "a" || food == "Amala and Egusi soup" {
      let calc:f32 = 2500.0 * portion_size;
      if calc > 10000.0 {
        Let calc:f32 = calc * 0.95;
        println!("Yayyyy, you get a discount. Your total cost is N{}", calc);break;
      }
    }
      else println!("Your total price is {}", calc);break;

    else if food == "e" || food == "Eba and Egusi soup" {
      let calc:f32 = 2000.0 * portion_size;
      if calc > 10000.0 {
        Let calc:f32 = calc * 0.95;
        println!("Yayyyy, you get a discount. Your total cost is N{}", calc);break;
      }
    
      else {println!("Your total price is {}", calc);break;}}

     else if food == "w" || food == "White Rice and Stew" {
      let calc:f32 = 2500.0 * portion_size;
      if calc > 10000.0 {
        let calc:f32 = calc * 0.95;
        println!("Yayyyy, you get a discount. Your total cost is N{}", calc);break;
      }
    
      else {println!("Your total price is {}", calc);break;}}

    }

}
}

