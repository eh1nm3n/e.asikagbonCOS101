use std::io;

fn main() {
    // Define the APS levels and roles for all professions
    let levels = vec![
        ("APS 1-2", vec![
            ("Office Administrator", "Intern"),
            ("Academic", "-"),
            ("Lawyer", "Paralegal"),
            ("Teacher", "Placement"),
        ]),
        ("APS 3-5", vec![
            ("Office Administrator", "Administrator"),
            ("Academic", "Research Assistant"),
            ("Lawyer", "Junior Associate"),
            ("Teacher", "Classroom Teacher"),
        ]),
        ("APS 5-8", vec![
            ("Office Administrator", "Senior Administrator"),
            ("Academic", "PhD Candidate"),
            ("Lawyer", "Associate"),
            ("Teacher", "Snr Teacher"),
        ]),
        ("EL1 8-10", vec![
            ("Office Administrator", "Office Manager"),
            ("Academic", "Post-Doc Researcher"),
            ("Lawyer", "Senior Associate 1-2"),
            ("Teacher", "Leading Teacher"),
        ]),
        ("EL2 10-13", vec![
            ("Office Administrator", "Director"),
            ("Academic", "Senior Lecturer"),
            ("Lawyer", "Senior Associate 3-4"),
            ("Teacher", "Deputy Principal"),
        ]),
        ("SES", vec![
            ("Office Administrator", "CEO"),
            ("Academic", "Dean"),
            ("Lawyer", "Partner"),
            ("Teacher", "Principal"),
        ]),
    ];

    // Ask user for profession
    let mut profession = String::new();
    println!("Enter profession (e.g., Lawyer, Teacher, Academic, Office Administrator):");
    io::stdin().read_line(&mut profession).expect("Failed to read line");
    let profession = profession.trim();

    // Ask user for title
    let mut title = String::new();
    println!("Enter title (e.g., Associate, Principal, Administrator):");
    io::stdin().read_line(&mut title).expect("Failed to read line");
    let title = title.trim();

    // Search for matching APS level
    let mut matched_level = None;
    for (level, roles) in &levels {
        for (prof, role_title) in roles {
            if prof.eq_ignore_ascii_case(profession) && role_title.eq_ignore_ascii_case(title) {
                matched_level = Some(level);
                break;
            }
        }
        if matched_level.is_some() {
            break;
        }
    }

    // Print result
    match matched_level {
        Some(level) => println!("✅ '{}' in '{}' is classified under: {}", title, profession, level),
        None => println!("❌ No valid APS level found for '{}' in '{}'.", title, profession),
    }
}
