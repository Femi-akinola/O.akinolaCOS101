use std::io;

fn main() {
    println!("Welcome to the Employee Incentive Checker!");

    // asks if the employee is experienced
    println!("Is the employee experienced? (yes or no):");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim().to_lowercase();

    // This asks for the age of the employee
    println!("Enter the age of the employee:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: u32 = age_input.trim().parse().expect("Please enter a valid number");

    // this checks the conditions and give the incentive
    if experience == "yes" {
        if age >= 40 {
            println!("Incentive is ₦1,560,000 per year.");
        } else if age >= 30 && age < 40 {
            println!("Incentive is ₦1,480,000 per year.");
        } else if age < 28 {
            println!("Incentive is ₦1,300,000 per month.");
        } else {
            println!("Incentive not specified for this age range.");
        }
    } else {
        println!("Incentive is ₦100,000.");
    }
}
