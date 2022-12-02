// Rust program to determine the annual incentive of an employee from his/her age and level of experience

use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
  
    println!("Number of years of experience: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experience:i32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter you age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if age >= 40 && experience >= 20 {
        println!("Your annual incentive is N1_560_000", );
    }
    else if age >= 30 && age < 40 && experience >= 10 && experience < 20 {
        println!("Your annual incentive is N1_480_000",);
    }
    else if age < 30 && experience >= 6 && experience < 10 {
        println!("Your annual incentive is N1_300_000",);
    }
    else if age >= 40 && experience <= 5 {
        println!("Your annual incentive is N100_000",);
    }
    else if age >= 30 && age < 40 && experience <=5 {
        println!("Your annual incentive is N100_000",);
    }
    else if age < 30 && experience <= 5 {
        println!("Your annual incentive is N100_000",); 
    }
}
