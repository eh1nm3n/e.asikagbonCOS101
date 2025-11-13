use std::io;

fn area_trapezium(){
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    println!("Enter the first base");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let base1:f32 = input1.trim().parse().expect("Invalid input");
    println!("Enter the second base");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let base2:f32 = input2.trim().parse().expect("Invalid input");
    println!("Enter the height");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let height:f32 = input3.trim().parse().expect("Invalid input");
    let area = (height / 2.0) * (base1 + base2);
    println!("The area of the trapezium is {}",area);
}
fn area_rhombus(){
    let mut input1 = String::new();
    let mut input2 = String::new();
    println!("Enter the first diagonal");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let diagonal1:f32 = input1.trim().parse().expect("Invalid input");
    println!("Enter the second diagonal");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let diagonal2:f32 = input2.trim().parse().expect("Invalid input");
    let area = 0.5 * diagonal1 * diagonal2;
    println!("The area of the rhombus is {}",area);
}
fn area_parallelogram(){
   let mut input1 = String::new();
   let mut input2 = String::new(); 
   println!("Enter the base");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let base:f32 = input1.trim().parse().expect("Invalid input");
    println!("Enter the altitude");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let altitude:f32 = input2.trim().parse().expect("Invalid input");
    let area = base * altitude;
    println!("The area of the parallelogram is {}",area);
}
fn area_cube(){
   let mut input1 = String::new(); 
   println!("Enter the length of side");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let side:f32 = input1.trim().parse().expect("Invalid input");
    let area = 6.0 * (side * side);
    println!("The area of the cube is {}",area);
}
fn volume_cylinder(){
   let mut input1 = String::new();
   let mut input2 = String::new(); 
   println!("Enter the radius");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let radius:f32 = input1.trim().parse().expect("Invalid input");
    println!("Enter the height");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let height:f32 = input2.trim().parse().expect("Invalid input");
    let volume = (22.0 / 7.0) * (radius * radius) * height;
    println!("The volume of a cylinder is {}",volume);
}
fn main(){
    println!("Area and Volume Calculator");
    println!("Choose what you want to calculate");
    println!("1.Area of a Trapezium");
    println!("2.Area of the Rhombus");
    println!("3.Area of Cube");
    println!("4.Area of parallelogram");
    println!("5.Volume of Cylinder");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice_new:u32 = choice.trim().parse().expect("Invalid input");
    if choice_new == 1 {
        area_trapezium();
    }
    else if choice_new == 2 {
        area_rhombus();
    }
    else if choice_new == 3 {
        area_cube();
    }
    else if choice_new == 4 {
        area_parallelogram();
    }
    else if choice_new == 5 {
        volume_cylinder();
    }
    else{
        println!("Invalid choice");
    }
}
