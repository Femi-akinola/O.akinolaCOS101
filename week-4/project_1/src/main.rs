use std::io;

fn main() {
    println!("Welcome to the Femi's Quadratic Equation Solver!");

    let a = input_coefficient("a");
    let b = input_coefficient("b");
    let c = input_coefficient("c");

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The equation has two distinct real roots: {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("The equation has exactly one real root: {}", root);
    } else {
        println!("There are no real roots. The roots are complex numbers.");
    }
}

fn input_coefficient(name: &str) -> f64 {
    loop {
        println!("Please enter the value of {}: ", name);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("That's not a valid number. Please try again."),
        }
    }
}