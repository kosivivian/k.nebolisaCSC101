// rust program that displays the following menu fpr the food items available to take order from the customer

use std::io;
fn main() {
   println!("Hello! Welcome to Kosi's kitchen");
   println!("Click enter to see the menu");


   
   let food = ["Poundo Yam/Edinkaiko Soup", "Fried Rice & Chicken ", "Amala & Ewedu Soup", "Eba & Egusi Soup", "White Rice & Stew"];
   let price = [3200.00, 3000.00, 2500.00, 2000.00, 2500.00];

   println!("-----------------Menu------------"); 
   for i in 0..food.len(){
      println!("{}\t\t{}", food[i], price[i]);
   }
   fn order_(){
    println!("if you want Poundo Yam/Edinkaiko Soup type 1.0\nif you want Fried Rice & Chicken type 2.0\nif you want Amala & Ewedu Soup type 3.0\nif you want Eba & Egusi Soup type 4.0\nif you want White Rice & Stew type 5.0"); 

   

   let P:f32 = 3200.00;
   let F:f32 = 3000.00;
   let A:f32 = 2500.00;
   let E:f32 = 2000.00;
   let W:f32 = 2500.00;

   let mut input1 = String::new();
   let mut input2 = String::new();

   println!("Place your order: ");
   io::stdin().read_line(&mut input1).expect("Not a valid string");
   let order:f32 = input1.trim().parse().expect("Not a valid number");

    println!("How many portions?");
   io::stdin().read_line(&mut input2).expect("Not a valid string");
   let portions:f32 = input2.trim().parse().expect("Not a valid number");
   

   if order == 1.0 {
   let mut price1:f32 = P * portions;
}
   else if order == 2.0 {
   let mut price2:f32 = F * portions;
}
   else if order == 3.0 {
   let mut price3:f32 = A * portions;
}
   else if order == 4.0 {
   let mut price4:f32 = E * portions;
}
   else if order == 5.0 {
   let  mut price5:f32 = W * portions;
}
   let mut total:f32 = price1 + price2 + price3 + price4 + price5;
   println!("The total cost is {}", total);

 println!("Do you want anything else?"); 
   }
    
    let mut input3 = String::new();

   println!("if you want something else type 7.0 or else type 8.0");   // for more orders
   io::stdin().read_line(&mut input3).expect("Not a valid string");
   let more:f32 = input3.trim().parse().expect("Not a valid number"); 
   if  more == 7.0 {
     order_();
     total =+ total;
   
   else if == 8.0 {
     
     println!("Your total order amount is {}",total);   // total of only one order
}
  if total > 10_000.0 {
   println!("You have a 5% discount!");
   let mut discount = (95 * total)/100;
   println!("Your final amount is {}",discount);
}
   else if total < 10_000.0 {
   println!("Thanks for shopping with us");
  }
} 










