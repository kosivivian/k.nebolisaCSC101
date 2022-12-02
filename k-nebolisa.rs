use std::io;

// merging separate arrays of into one output
fn geopo_merger(){
   let commisioner_name:[&str;5] = ["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
   let ministry:[&str;5] = ["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
   let zone:[&str;5] = ["South West","North East","South South","South West","South East"];

   for i in 0..commisioner_name.len() {
     println!("{}\t\t{}\t\t{}", commisioner_name[i],ministry[i],zone[i]);
   } 
fn pub_service(){
	let public_servant:[&str;6] = ["1-2","3-5","5-8","8-10","10-13","14"];
	let office_administrator:[&str;6] = ["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
	let academic:[&str;6] = ["-","Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
	let lawyer:[&str;6] = ["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
	let teacher:[&str;6] = ["Placement","Classroom Teacher","Snr Teacher","Leading Teacher","Deputy Principal","Principal"];

	 for i in 0..public_servant.len() {
        println!("{}\t\t{}\t\t{}\t\t{}\t\t{}\t\t{}", public_servant[i],office_administrator[i],academic[i],lawyer[i],teacher[i]);
     }

 // to validate and check staff employment positions
  println!("Welcome to the Public service division");
  let mut input2 = String::new();
	println!("what is the staff name?");
	io::stdin().read_line(&mut input1).expect("Not a valid string");
   let name = input2.trim().parse().expect("Not a valid number");
  let mut input3 = String::new();
	println!("Is the staff a public servant? (y-3.0 or n-4.0)");
	io::stdin().read_line(&mut input3).expect("Not a valid string");
   let ps:f32 = input3.trim().parse().expect("Not a valid number");
  let mut input4 = String::new();
	println!("Is the staff an administrative officer? (y-3.0 or n-4.0) ");
	io::stdin().read_line(&mut input4).expect("Not a valid string");
   let service:f32 = input4.trim().parse().expect("Not a valid number");
  let mut input5 = String::new();
	println!("What is the number of years of experience?");
	io::stdin().read_line(&mut input5).expect("Not a valid string");
   let years:f32 = input5.trim().parse().expect("Not a valid number");

   if ps == 3.0 && service == 3.0 && years == 1.0-2.0 {
     println!("{} is  paralegal",name,);
   }

   else if ps == 3.0 && service == 3.0 && years == 3.0-5.0 {
     println!("{} is  paralegal",name,);
   }

   else if ps == 3.0 && service == 3.0 && years == 5.0-8.0 {
     println!("{} is  paralegal",name,);
   }

   else if ps == 3.0 && service == 3.0 && years == 8.0-10.0 {
     println!("{} is  paralegal",name,);
   }

   else if ps == 3.0 && service == 3.0 && years == 10.0-13.0 {
     println!("{} is  paralegal",name,);
   }

   else if ps == 3.0 && service == 3.0 && years == 14.0 {
     println!("{} is  paralegal",name,);
   }
}
fn main() {
	println!("Welcome to The Office of The Federal Government of Nigeria");
	println!("We provide the following division services:
		      1.0 Geopolitical zoning division services     
		      2.0 Public Service division");

	let mut input1 = String::new();
	println!("Which of the services are you here for?");
	io::stdin().read_line(&mut input1).expect("Not a valid string");
   let service:f32 = input1.trim().parse().expect("Not a valid number");

   if service == 1.0 {
   	geopo_merger();
   }
   if service == 2.0 {
    pub_service();
   }
}