use std::io::Write;
use std::io::Read;
fn main(){
let mut file_1 = std::fs::File::create("welcome_message.txt").expect("create failed");
    file_1.write_all("Welcome to error messages\n"
    .as_bytes()).expect("write failed");
let mut file = std::fs::File::open("welcome_message.txt").unwrap();
let mut contents = String::new();
file.read_to_string(&mut contents).unwrap();
print!("{}",contents);
}