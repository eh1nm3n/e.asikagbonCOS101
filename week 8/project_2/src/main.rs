fn main(){
    let developers = vec!["Jake","Chris","Danielle","Mikaela"];
    let years = vec![5,7,10,15];
    let mut highest = years[0];
    let mut index = 0;
    for i in 0..years.len(){
        if years[i] > highest{
            highest = years[i];
            index = i;
        }
    }
    println!("Developers with the highest experience is {} with {} years",developers[index],highest);
}
