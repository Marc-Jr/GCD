use gcd::Gcd;
use std::io::{self, Read, Write}; 

fn main() {
    gcd1();

    let mut input_c = String::new();
    println!("Do you want to find the gcd of two other numbers?(yes/no)");
   
    io::stdin().read_line(&mut input_c).unwrap_or(0);
    
    let c = input_c.trim().to_lowercase();
    if c == "yes" {
        gcd1();
    }
    else if c == "no" {
        println!("See you next time");
    }
    else {
        println!("Enter a 'yes' or 'no' ");
    }
}

fn gcd1() {
        // Get the first number (a) from the user
        print!("Enter the first number: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately
        let mut input_a = String::new();
        io::stdin().read_line(&mut input_a).unwrap();
        let a: u32 = input_a.trim().parse().unwrap_or(0); // Parse the input into an integer, default to 0 if parsing fails
    
        // Get the second number (b) from the user
        print!("Enter the second number: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately
        let mut input_b = String::new();
        io::stdin().read_line(&mut input_b).unwrap();
        let b: u32 = input_b.trim().parse().unwrap_or(0); // Parse the input into an integer, default to 0 if parsing fails
    
        // Compute the GCD of a and b
        let result = a.gcd(b);
    
        // Display the result
        println!("GCD of {} and {} is {}", a, b, result);
}




























