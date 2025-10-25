use std::io;
fn main() {
    let mut experience = String::new();
    let mut age = String::new();
    println!("\nIs the employee experienced? (yes or no): ");
    io::stdin().read_line(&mut experience).expect("Not a valid string");
    println!("\nEnter employee's age: ");
    io::stdin().read_line(&mut age).expect("Not a valid string");
    let age:i32 = age.trim().parse().expect("Not a valid number");
    if age >= 40{
        println!("The employee's incentive is 1,560,000");
    }
    else if age >= 30 && age < 40{
        println!("The employee's incentive is 1,480,000");
    }
    else if age <= 28 && age >= 19{
        println!("The employee's incentive is 1,300,000");
    }
    else {
        println!("The employee's incentive is 100,000");
    }

    
}
