fn main() {
    let p: f64 = 510_000.00; 
    let r: f64 = 5.0;         
    let t: f64 = 3.0;           

    // depreciation
    let a = p * (1.0 - (r / 100.0)).powf(t);
    println!("Amount is {}", a);

}