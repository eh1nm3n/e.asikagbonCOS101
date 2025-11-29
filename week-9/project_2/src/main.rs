
use std::io::Write;

fn main() {
    // Separate vectors for each attribute
    let names = vec![
        "Oluchi Mordi",
        "Adams Aliyu",
        "Shania Bolade",
        "Adekunle Gold",
        "Blanca Edemoh"
    ];
    let matric_numbers = vec![
        "ACC10211111",
        "ECO10110101",
        "CSC10328828",
        "EEE11020202",
        "MEE10202001"
    ];
    let departments = vec![
        "Accounting",
        "Economics",
        "Computer",
        "Electrical",
        "Mechanical"
    ];
    let levels = vec![300, 100, 200, 200, 100];

    // Display student records
    println!("PAU SMIS Student Records:");
    for i in 0..names.len() {
        println!(
            "{} | {} | {} | Level {}",
            names[i], matric_numbers[i], departments[i], levels[i]
        );
    }

    // Write to file
    let mut file = std::fs::File::create("students.txt").expect("Could not create file");

    file.write_all("Student Name\tMatric Number\tDepartment\tLevel\n".as_bytes())
        .expect("Write failed");

    for i in 0..names.len() {
        let line = format!(
            "{}\t{}\t{}\t{}\n",
            names[i], matric_numbers[i], departments[i], levels[i]
        );
        file.write_all(line.as_bytes()).expect("Write failed");
    }

    println!("File has been saved successfully");
}
