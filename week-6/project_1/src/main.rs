use std::io;
fn main() {
    println!("Food Menu!!!");
    println!("P--Poundo Yam/Edinkaiko Soup -----#3200.00");
    println!("F--Fried Rice & Chicken -----#3000.00");
    println!("A--Amala & Ewedu Soup -----#2500.00");  
    println!("E--Eba & Egusi Soup -----#2000.00");
    println!("W--White Rice & Stew -----#2500.00");
    println!("What would you like to order? (P/F/A/E/W)");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let food = input1.trim().to_uppercase();
    println!("How many {} items would you like to order",food);
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let quantity:f64 = input2.trim().parse().expect("Invalid input");
    let mut price = 0.0;
    if food == "P"{
        price = 3200.0;
    }
    else if food == "F"{
        price = 3000.0;
    }
    else if food == "A"{
        price = 2500.0;
    }
    else if food == "E"{
        price = 2000.0;
    }
    else if food == "W"{
        price = 2500.0;
    }
    else{
        println!("Invalid food code entered");
        return;
    }
    let mut total = price * quantity;
    if total > 10000.0{
        let discount = total * 0.05;
        println!("You got a 5% discount of {} naira",discount);
        total = total - discount;
    }
    println!("The total cost of your order is {} naira",total);
    
    
}
