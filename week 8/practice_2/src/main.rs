fn main(){
    let f = vec!['C','0','P','M', 'U','E','T','R'];
    let mut input1 = String::new();
    println!("Enter an index value btw (0 - 7)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let index:usize = input1.trim().parse().expect("Invalid input");
    let ch: char = f[index];
    print!("{} is the character for index[{}]\n",ch,index);
}
