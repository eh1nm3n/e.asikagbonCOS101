use std::io;
use std::io::Read;
fn database(){
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
fn project(){
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents1 = String::new();
    file.read_to_string(&mut contents1).unwrap();
    print!("{}",contents1);
}
fn staff(){
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents2 = String::new();
    file.read_to_string(&mut contents2).unwrap();
    print!("{}",contents2);
}
fn customer(){
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents3 = String::new();
    file.read_to_string(&mut contents3).unwrap();
    print!("{}",contents3);
}
fn dataplan(){
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents4 = String::new();
    file.read_to_string(&mut contents4).unwrap();
    print!("{}",contents4);
}
fn main(){
    println!("Enter your role: Administrator, project manager, customer, vendor, employee");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    let role = input.trim().to_lowercase();
    match role.as_str() {
        "administrator" => database(),
        "project manager" => project(),
        "employee" => staff(),
        "customer" => customer(),
        "vendor" => dataplan(),
         &_ => todo!(),
    };
}
