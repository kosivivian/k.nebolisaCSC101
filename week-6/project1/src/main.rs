use std::io;

fn trapezium(){
    let mut input1 = String::new();
        println!("Type in the height");
        io::stdin().read_line(&mut input1).expect("Unable to read input");
        let height:f32 = input1.trim().parse().expect("Not a valid number");

    let mut input2 = String::new();
        println!("Type in the base1");
        io::stdin().read_line(&mut input2).expect("Unable to read input");
        let base1:f32 = input2.trim().parse().expect("Not a valid number");

    let mut input3 = String::new();
        println!("Type in the base2");
        io::stdin().read_line(&mut input3).expect("Unable to read input");
        let base2:f32 = input3.trim().parse().expect("Not a valid number");

    let area_trapezium:f32 = height/2.0 + (base1 + base2);
    println!("The area of the trapezium is {}",area_trapezium);
}
fn rhombus(){
   let mut input4 = String::new();
        println!("Type in the diagonal1");
        io::stdin().read_line(&mut input4).expect("Unable to read input");
        let diagonal1:f32 = input4.trim().parse().expect("Not a valid number");

    let mut input5 = String::new();
        println!("Type in the diagonal2");
        io::stdin().read_line(&mut input5).expect("Unable to read input");
        let diagonal2:f32 = input5.trim().parse().expect("Not a valid number");

    let area_rhombus:f32 = 1.0/2.0 * (diagonal1 * diagonal2);
    println!("The area of the rhombus is {}",area_rhombus); 
}
fn parallelogram(){
   let mut input6 = String::new();
        println!("Type in the base");
        io::stdin().read_line(&mut input6).expect("Unable to read input");
        let base:f32 = input6.trim().parse().expect("Not a valid number");

    let mut input7 = String::new();
        println!("Type in the altitude");
        io::stdin().read_line(&mut input7).expect("Unable to read input");
        let altitude:f32 = input7.trim().parse().expect("Not a valid number");

    let area_parallelogram:f32 = base * altitude;
    println!("The area of the parallelogram is {}",area_parallelogram);
 
}
fn cube(){
    let mut input8 = String::new();
        println!("Type in the length");
        io::stdin().read_line(&mut input8).expect("Unable to read input");
        let lengthside:f32 = input8.trim().parse().expect("Not a valid number");
       
      let k = f32::powf(lengthside,2.0);
    let area_cube:f32 = 6.0 * k;
    println!("The area of the parallelogram is {}",area_cube);
 
}
fn cylinder(){
    let mut input9 = String::new();
        println!("Type in the radius");
        io::stdin().read_line(&mut input9).expect("Unable to read input");
        let radius:f32 = input9.trim().parse().expect("Not a valid number");

    let mut input10 = String::new();
        println!("Type in the height");
        io::stdin().read_line(&mut input10).expect("Unable to read input");
        let height:f32 = input10.trim().parse().expect("Not a valid number");
    let pi:f32 = 22.00/7.00;
    let g = f32::powf(radius,2.0);
    let area_cylinder:f32 = pi * g * height;
    println!("The area of the parallelogram is {}",area_cylinder); 
}


fn solve(){   
    let mut num = String::new();
    println!("------What would you like to calculate------
              Area of Trapezium formula          1
              Area of the rhombus formula        2
              Area of Parallelogram formula      3
              Area of Cube formula               4
              Volume of Cylinder formula         5 ");
    io::stdin().read_line(&mut num).expect("Invalid input");
    let calculation:i32 = num.trim().parse().expect("Not a valid number");

  if calculation == 1 {
     trapezium()
  }
  if calculation == 2 {
     rhombus()
  }
  if calculation == 3 {
     parallelogram()
  }
  if calculation == 4 {
     cube();
  }
  if calculation == 5 {
     cylinder();
  }
}

fn main() {
    println!("Welcome to the Calculator!");
    println!("We help in the calculation of area and volume of solid shapes");
    solve();

  let mut num1 = String::new();
  println!("Do you want to do more calculations? (y or n)");
  io::stdin().read_line(&mut num1).expect("Unable to read input");
  let more:&str = num1.trim()
  
  if more == "y" {
  while more == "y" {
    solve();
  }
  }
  println!("end");


}
