use std::io;


fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the coefficient of x-squared");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid value");

    println!("Enter the coefficient of x");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid value");

    println!("Enter the constant");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid value");

    let d:f32 = (b * b) - (4.0 * a * c);

    let root_1:f32 = (-b + d.sqrt()) / (2.0 * a);
    let root_2:f32 = (-b - d.sqrt()) / (2.0 * a);

    println!("Root 1 is: {}", root_1);
    println!("Root 2 is: {}", root_2);

    if d == 0.0 {
        println!("there is one real root.");
    }
    else if d > 0.0 {
        println!("There are two distict roots");
    }
    else if d < 0.0 {
        println!("There are no real roots");
    }


}
