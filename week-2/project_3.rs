<<<<<<< HEAD
fn main() {
	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	// Amount of the tv three years later
	let a = p * (1.0 - (r / 100.0)).powf(t);
	println!("Amount is {}", a);

	// Depreciation
	let d = a - p;
	println!("Depreciation is {}", d);
=======
fn main() {
	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	// Amount of the tv three years later
	let a = p * (1.0 - (r / 100.0)).powf(t);
	println!("Amount is {}", a);

	// Depreciation
	let d = a - p;
	println!("Depreciation is {}", d);
>>>>>>> 7fb5120708eb4fac2eef0b86ec94c7e479bd0398
}