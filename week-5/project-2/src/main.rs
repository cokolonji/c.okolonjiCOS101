use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();


    println!("\nAre you experienced:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experience = input1.trim().parse().expect("Not a valid number");


    println!("\nEnter your age:");
    io::stdin().read_line(%mut input2).expect("Not a valid string");
    let age:f32 = input2.trim().parse().expect("Not a valid number");

    if experience == "yes"  && age >= 40.0{
        println!("Incentive is N1,560,000 ");
    }
    else if experience == "yes" && age >= 30.0 && age < 40.0{
        println!("Incentive is N1,480,000");
    } 
    else if experience == "yes" && age < 28.0{
        println!("Incentive is N1,300,000");
    }
    else if experience == "no"{
        println!("Incentive is N100,000");
    }
    

}
