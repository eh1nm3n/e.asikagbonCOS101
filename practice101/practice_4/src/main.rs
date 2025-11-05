use std::io;
fn main(){
    let mut input1 = String::new();
    let mut input2 = String::new();
    loop{
        println!("Enter the item preffered");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let item = input1.trim().to_uppercase();
        println!("Enter the number of {} item you want to purchase", item);
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let q:f32 = input2.trim().parse().expect("Invalid input");
        if item == "L"{
            let cost:f32 = 550_000.0 * q;
            println!("The total cost of items is {}",cost);
        }
        else if item == "M"{
            let cost:f32 = 400_000.0 * q;
            println!("The total cost of items is {}",cost);
        }
        else if item == "K"{
            let cost:f32 = 300_000.0 * q;
            println!("The total cost of items is {}",cost);
        }
        else if item == "H"{
            let cost:f32 = 200_000.0 * q;
            println!("The total cost of items is {}",cost);
        }
        else{
            println!("Invalid!!");
        }
        let mut choice = String::new();
        println!("Would you like to continue the program Y/N");
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice_new = choice.trim().to_uppercase();
        if choice_new == "N"{
            break;
        }

    }
}
