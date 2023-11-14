<<<<<<< HEAD
fn main() {
	
	// x is amount

	let tx:f64 = 450000.0;
	let mx:f64 = 1500000.0;
	let hx:f64 = 750000.0;
	let dx:f64 = 2850000.0;
	let ax:f64 = 250000.0;

	// f is quantity

	let tf:f64 = 2.0;
	let mf:f64 = 1.0;
	let hf:f64 = 3.0;
	let df:f64 = 3.0;
	let af:f64 = 1.0;

	// fx

	let tfx:f64 = tf * tx;
	let mfx:f64 = mf * mx;
	let hfx:f64 = hf * hx;
	let dfx:f64 = df * dx;
	let afx:f64 = af * ax;

	// total sales amount
	let t = tfx + mfx + hfx + dfx + afx;
	println!("Total number of items sold is {}", t);

	// total number of items old
	let n = tf + mf + hf + df + af;
	println!("Total number of items sold is {}", n);

	// average of sales record
	let a = t / n;
	println!("Average of sales records is {}", a);
=======
fn main() {
	
	// x is amount

	let tx:f64 = 450000.0;
	let mx:f64 = 1500000.0;
	let hx:f64 = 750000.0;
	let dx:f64 = 2850000.0;
	let ax:f64 = 250000.0;

	// f is quantity

	let tf:f64 = 2.0;
	let mf:f64 = 1.0;
	let hf:f64 = 3.0;
	let df:f64 = 3.0;
	let af:f64 = 1.0;

	// fx

	let tfx:f64 = tf * tx;
	let mfx:f64 = mf * mx;
	let hfx:f64 = hf * hx;
	let dfx:f64 = df * dx;
	let afx:f64 = af * ax;

	// total sales amount
	let t = tfx + mfx + hfx + dfx + afx;
	println!("Total number of items sold is {}", t);

	// total number of items old
	let n = tf + mf + hf + df + af;
	println!("Total number of items sold is {}", n);

	// average of sales record
	let a = t / n;
	println!("Average of sales records is {}", a);
>>>>>>> 7fb5120708eb4fac2eef0b86ec94c7e479bd0398
	}