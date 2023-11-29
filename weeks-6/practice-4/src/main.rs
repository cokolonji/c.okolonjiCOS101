use std::io

fn add(a:i32, b:i32) {
    let sum = a + b;
    println!("Sum of A and B = {}", sum);

}

fn main() {
    let mut input1 = String::new();
    println!("Enter input for the parameter A:");
    io::stdin().read_line(&mut input1).expect("not a valid input");
    let a:i32 = input1.trim().parse().expect("Not a valid input");

    let mut input2 = String::new();
    println!("Enter input for the parameter A:");
    io::stdin().read_line(&mut input2).expect("not a valid input");
    let b:i32 = input2.trim().parse().expect("Not a valid input");

    //call add functions with arguements
    add(a,b);
}