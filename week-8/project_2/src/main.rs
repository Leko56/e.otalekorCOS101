// Project II: The EY Interview
// Finds the candidate with the highest years of programming experience.
// Compound data type used: a Vec of tuples, (name, years).

use std::io;

// Read one trimmed line from the keyboard
fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

// Keep asking until the user types a valid whole number
fn read_number(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);
        match read_line().parse::<u32>() {
            Ok(n) => return n,
            Err(_) => println!("Please enter a whole number (0 or more)."),
        }
    }
}

fn main() {
    println!("=== EY Nigeria: Developer Interview ===\n");

    let total = read_number("How many candidates are you interviewing?");

    // Each candidate is a tuple: (name, years of programming experience)
    let mut candidates: Vec<(String, u32)> = Vec::new();

    for i in 0..total {
        println!("\nCandidate {}", i + 1);
        println!("Name:");
        let name = read_line();
        let years = read_number("Years of programming experience:");
        candidates.push((name, years));
    }

    if candidates.is_empty() {
        println!("\nNo candidates were entered.");
        return;
    }

    // Walk the vector, keeping the highest years seen so far
    let mut highest = candidates[0].1;
    for candidate in &candidates {
        if candidate.1 > highest {
            highest = candidate.1;
        }
    }

    // Collect everyone who matches the highest (handles ties)
    println!("\n--- Result ---");
    let mut winners = 0;
    for (name, years) in &candidates {
        if *years == highest {
            println!("{} has the highest experience: {} years", name, years);
            winners += 1;
        }
    }

    if winners > 1 {
        println!("({} candidates are tied.)", winners);
    }
}