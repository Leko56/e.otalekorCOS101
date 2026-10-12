use std::io;

// Small helper: print a prompt, read one line, return it trimmed (Practice 5 style)
fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    // Practice 1: vectors created with the vec! macro.
    // One vector holds the levels; four more hold the job titles for each profession.
    // Position 0 in every vector lines up with APS 1-2, position 1 with APS 3-5, and so on.
    let levels: Vec<&str> = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    let office_admin: Vec<&str> = vec![
        "Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO",
    ];
    let academic: Vec<&str> = vec![
        "-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean",
    ];
    let lawyer: Vec<&str> = vec![
        "Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2",
        "Senior Associate 3-4", "Partner",
    ];
    let teacher: Vec<&str> = vec![
        "Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher",
        "Deputy Principal", "Principal",
    ];

    println!("=== Federal Public Service APS Level Checker ===");

    // Practice 5: an empty vector that grows at runtime, one result per staff member
    let mut report: Vec<String> = Vec::new();

    let staff_count: u32 = read_input("How many staff members do you want to check?")
        .parse()
        .expect("Invalid input");

    for n in 0..staff_count {
        println!("\n--- Staff member {} ---", n + 1);

        let sector: u32 = read_input(
            "Choose a profession:\n1. Office Admin\n2. Academic\n3. Lawyer\n4. Teacher",
        )
        .parse()
        .expect("Invalid input");

        // Pick which titles vector to search
        let titles: &Vec<&str> = match sector {
            1 => &office_admin,
            2 => &academic,
            3 => &lawyer,
            4 => &teacher,
            _ => {
                println!("Invalid profession number.");
                continue;
            }
        };

        let job = read_input("Enter the job title (e.g. Associate):");
        let claimed = read_input("Enter the APS level claimed (e.g. APS 5-8):");

        // Practice 4: loop over the vector with an index.
        // -1 means "not found yet".
        let mut position: i32 = -1;
        for i in 0..titles.len() {
            // Practice 2: subscript access, safe because i stays below titles.len()
            if titles[i] != "-" && titles[i].to_lowercase() == job.to_lowercase() {
                position = i as i32;
            }
        }

        if position < 0 {
            println!("'{}' is not a recognised title for that profession.", job);
            report.push(format!("{}: NOT FOUND", job));
            continue;
        }

        // Guard first (Practice 2): position is non-negative and inside the vector
        let correct_level = levels[position as usize];

        if claimed.to_lowercase() == correct_level.to_lowercase() {
            println!("VALID: {} holds {}.", job, correct_level);
            report.push(format!("{}: VALID ({})", job, correct_level));
        } else {
            println!(
                "INVALID: {} should be {}, not {}.",
                job, correct_level, claimed
            );
            report.push(format!("{}: INVALID (correct level is {})", job, correct_level));
        }
    }

    // Final summary, iterating over the vector we built
    println!("\n=== Summary ===");
    let mut count = 1;
    for line in &report {
        println!("{}. {}", count, line);
        count += 1;
    }
}