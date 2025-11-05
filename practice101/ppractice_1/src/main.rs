use std::io;
fn main(){
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    loop{
    println!("Enter the principal");
    io::stdin().read_line(&mut input1).expect("failed to read input");
    let p:f32 = input1.trim().parse().expect("Invalid input");
    
    println!("Enter the rate");
    io::stdin().read_line(&mut input2).expect("failed to read input");
    let r:f32 = input2.trim().parse().expect("Invalid input");
    
    println!("Enter the time");
    io::stdin().read_line(&mut input3).expect("failed to read input");
    let t:f32 = input3.trim().parse().expect("Invalid input");
    
    let a = p * (1.0 + (r / 100.0)) * t;
    let ci = a - p;
    println!("The compound interest is {}", ci);
    
        println!("Would you like to print for another customer? (y / n)");
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("failed to read input");
        let choice = input4.trim().to_uppercase();
        
        if choice =="n"{
           break;
        }
        
    }
}
