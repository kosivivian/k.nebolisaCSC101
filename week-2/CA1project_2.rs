fn main() {
	let T:f64 = 450_000.00;
	let M:f64 = 1_500_000.00;
	let H:f64 = 750_000.00;
	let D:f64 = 2_850_000.00;
	let A:f64 = 250_000.00;

	// sum
	let Sum = (T * 2.0) + (M * 1.0) + (H * 3.0) + (D * 3.0) + (A * 1.0);
	println! ("sum is {}", Sum);
	let TQ = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;
	let AVE = Sum / TQ;
	println! ("average is {}", AVE);
}