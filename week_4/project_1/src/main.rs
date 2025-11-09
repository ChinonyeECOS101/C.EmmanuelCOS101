// Rust program to find the roots of a quadratic equation

use std::io;

fn main(){
    println!("Quadratic Equation Slover");
    println!("Equation Form: ax²+bx+c");

    // input values of the equation

    let mut inputa = String::new();
    let mut inputb = String::new();
    let mut inputc = String::new();

    println!("Input the coefficient of x²");
    io::stdin().read_line(&mut inputa).expect("Not a valid string");
    let a:f32 = inputa.trim().parse().expect("Invalid number"); 


    println!("Input the coefficient of x");
    io::stdin().read_line(&mut inputb).expect("Not a valid string");
    let b:f32 = inputb.trim().parse().expect("Invalid number"); 

    println!("Input the costant");
    io::stdin().read_line(&mut inputc).expect("Not a valid string");
    let c:f32 = inputc.trim().parse().expect("Invalid number");

    // Compute discriminant

    let rita = b * b - 4.0 * a * c;
    println!("\nDiscriminant (D) = {}",rita);

    if rita >= 0.0{
        let lab1 = (-b + rita.sqrt()) / (2.0 * a);
        let lab2 = (-b - rita.sqrt()) / (2.0 * a);
        println!("There are two distinct roots, which are:");
        println!("X₁ = {}",lab1);
        println!("X₂ = {}",lab2);
    }else if rita == 0.0{
        let lab1 = (-b + rita.sqrt()) / (2.0 * a);
        let lab2 = (-b - rita.sqrt()) / (2.0 * a);
        println!("There is only one real root");
        println!("X₁ = {}",lab1);
        println!("X₂ = {}",lab2);
    }else {
        let lab1 = (-b + rita.sqrt()) / (2.0 * a);
        let lab2 = (-b - rita.sqrt()) / (2.0 * a);
        println!("The Discriminant is negative.");
        println!("There is no real roots/ complex roots exist");
        println!("X₁ = {}",lab1);
        println!("X₂ = {}",lab2);
    }

}
