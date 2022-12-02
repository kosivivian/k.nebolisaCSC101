use std::io;

fn student_council_votex() {

  for 1 in 0..15{
    let mut input4 = String::new();
    println!("\nWhat is your name? {}",1);
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let mut name = input4.trim();

    let mut input5 = String::new();
    println!("\nWhat is your email?");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let mut email = input5.trim();

    let mut input6 = String::new();
    println!("\nWhat is your department?");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let mut department = input6.trim();

    let mut input7 = String::new();
    println!("\nWhat is your state of origin?");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let mut state = input7.trim();

    // to ask if person is current class rep
    let mut input1 = String::new(); 
    println!("\n;plAre you a current class rep? (yes or no)");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let mut rep = input1.trim();


     // to ask if the person is in 100 level
    let mut input2 = String::new();
    println!("\nAre you in 100 level? (yes or no)");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let mut level = input2.trim();

    // to ask for GPA
    let mut input3 = String::new();
    println!("\nWhat is your GPA?");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let mut gpa:f32 = input3.trim().parse().expect("Invalid number");


    //for calling the function
    if (rep == "yes" || rep == "YES") && (level == "no" || level == "NO") & (gpa > 4.0) 
    {
      println!("Name: {}
      	Email: {}
      	Department: {}
      	State_of_Origin: {}",name,email,department,state);
      println!("You can vote!");
    }
    else {
   	  println!("Sorry you are not eligible");
   }
}
}

fn main() {
	println!("Good day! Please kindly answer the following questions to check your eligibility to vote");

	student_council_votex()




}
	
