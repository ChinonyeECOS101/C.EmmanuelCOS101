// Rust program that shows employee annual incentive


 use std::io;

fn main(){
    println!("Employee Annual Incentive Form");

    // input details of the employee

    let mut experience = String::new();
    let mut agi_input = String::new();

    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut experience).expect("Invaild answer");
    let experience = experience.trim().to_uppercase();

    println!("Enter employer's age");
    io::stdin().read_line(&mut agi_input).expect("Input error");
    let agi:u32 = agi_input.trim().parse().expect("Invalid figure");   

    //Determine annual incentive

    let incent:u32;

    if experience == "yes"{
        if agi >= 40 {
            incent = 1_560_000;
        }else if agi >= 30 {
            incent = 1_480_000;   
        }else if agi < 28 {
            incent = 1_300_000;
        }else {
            // if the agi is between 28 and 29 (not covered above), assume same as < 28
            incent = 1_300_000
        }
    }else {
        incent = 100_000;
    }


    // Rust Display

    println!("The employee's annual Incentive is ₦{}",incent);
}