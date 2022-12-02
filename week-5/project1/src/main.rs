// Rust program to find roots of a quadratic equation

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Input a:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");
    
     println!("Input b:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");
    
    println!("Input c:");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let d = f32::powf(b,2.0);   // making b a squared number
    let mut roots = d - (4.0*a*c);   // calculate discriminant
    
    if roots > 0.0 
    {
        println!("Discriminant: {}",roots );
    println!("There are two distinct roots");
}
    else if roots == 0.0
    {
        println!("Discriminant: {}", roots);
    println!("There is one real root");
}
    else if roots < 0.0
    {
        println!("Discriminant: {}", roots);
    println!("There are no real roots");
}
    


}
