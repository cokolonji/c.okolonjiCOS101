fn main() {
	let p = 520000000.00;
	let r = 5.0;
	let t = 10.0;

	// simple interest
	let a = p * (1.0 + (r/100.0)) * t;
	println!("Amount is {}", a);

	// compound interest
	let ci = a - p;
	println!("Compound Interest is {}", ci);
}