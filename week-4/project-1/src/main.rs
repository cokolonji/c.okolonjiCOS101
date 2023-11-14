use std::io;


fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter Distance car made");
    io::stdin().read_line(&mut input1).expect("Not a valid number");
    let a:f32 = input1.trim().parse().expect("Not a valid number");


    println!("Enter Time car completed Distance");
    io::stdin().read_line(&mut input2).expect("Not a valid number");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    let speed:f32 = a / b;

    println!("Speed of Car: {}", speed);




}
