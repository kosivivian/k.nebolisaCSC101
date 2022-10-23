fn main() {
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

	// compound interest
	let A = 1.0 + (r / 100.0);
	let Y = A * A * A * A * A;
	let CI = (p * Y) - p;
	println!("compound interest is {}", CI);
}