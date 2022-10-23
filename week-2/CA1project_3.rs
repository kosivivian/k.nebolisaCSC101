fn main() {
	let p:f64 = 210_000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	// amount after three years 
	let D = 1.00 - (r / 100.00);
	let Y = D * D * D;
	let A = p * Y;
	println!("Amount after three years is {}", A)
}