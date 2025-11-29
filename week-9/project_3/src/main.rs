
use std::io::Write;

fn main() {
    let names = vec![
        "Aiobogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogobona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieve"
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum"
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East"
    ];

    // Display header
    println!("{:<30} {:<20} {:<15}", "Name", "Ministry", "Geopolitical Zone");

    // Display aligned records
    for i in 0..names.len() {
        println!(
            "{:<30} {:<20} {:<15}",
            names[i], ministries[i], zones[i]
        );
    }

    // Write to file
    let mut file = std::fs::File::create("efcc_report.txt").expect("Could not create file");
    let header = format!("{:<30} {:<20} {:<15}\n", "Name", "Ministry", "Geopolitical Zone");
    file.write_all(header.as_bytes()).expect("Write failed");

    for i in 0..names.len() {
        let line = format!(
            "{:<30} {:<20} {:<15}\n",
            names[i], ministries[i], zones[i]
        );
        file.write_all(line.as_bytes()).expect("Write failed");
    }

    println!("\nAligned report saved to efcc_report.txt");
}
