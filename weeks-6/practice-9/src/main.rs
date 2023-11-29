//the iter() function fetches values of all eements in the array

fn main() {
    let arr:[i32;4] = [10,20,30,40]
    println!("The array is; {}", arr);
    println!("The array size is: {}", arr.len());

    for val in arr.iter(){
        println!("Value is: {}", val);
    }
}
