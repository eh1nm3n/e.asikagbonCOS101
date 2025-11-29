use std::io::Write;

fn main() {
    let lager: Vec<&str> = vec![
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star"
    ];
    let stout: Vec<&str> = vec![
        "Legend",
        "Turbo King",
        "Williams"
    ];
    let non_alcoholic: Vec<&str> = vec![
        "Maltina",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz"
    ];

    let mut file = std::fs::File::create("breweries.txt").expect("create failed");

    // Lager block
    file.write_all("Lager:\n".as_bytes()).expect("write failed");
    for drink in &lager {
        file.write_all(format!("{}\n", drink).as_bytes()).expect("write failed");
    }

    // Stout block
    file.write_all("\nStout:\n".as_bytes()).expect("write failed");
    for drink in &stout {
        file.write_all(format!("{}\n", drink).as_bytes()).expect("write failed");
    }

    // Non-Alcoholic block
    file.write_all("\nNon-Alcoholic:\n".as_bytes()).expect("write failed");
    for drink in &non_alcoholic {
        file.write_all(format!("{}\n", drink).as_bytes()).expect("write failed");
    }

    println!("file created successfully");
}
