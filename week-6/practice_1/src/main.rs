fn main() {
   let name = "Femi Otedola";
   let uni:&str = "Pan-Atlantic University";
   let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
   println!("Name: {}", name);
   println!("University: {}, \nAddress: {}",uni,addr);


   let mut department:&'static str = "Computer Science";
   let mut school:&'static str = "School of Science and Technology";
   println!("Department: {}, \nSchool: {}",department,school);
}