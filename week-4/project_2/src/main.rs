use std::io;

fn main() {
    // 1. Read whether the employee is experienced
    println!("Is the employee experienced? (yes/no):");
    let mut exp_input = String::new();
    io::stdin()
        .read_line(&mut exp_input)
        .expect("Failed to read input");
    let experienced: bool = exp_input.trim().eq_ignore_ascii_case("yes");

    // 2. Read the employee's age
    println!("Enter the employee's age:");
    let mut age_input = String::new();
    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read input");
    let age: u32 = age_input.trim().parse().expect("Input not an integer");

    // 3. Decide the incentive with an else if ladder
    let incentive: u32;

    if !experienced {
        incentive = 100_000;
    } else if age >= 40 {
        incentive = 1_560_000;
    } else if age >= 30 {
        // covers 30 to 39, since the age >= 40 case was already handled above
        incentive = 1_480_000;
    } else if age < 28 {
        incentive = 1_300_000;
    } else {
        // age is 28 or 29: experienced, but not covered by any listed band
        incentive = 0;
    }

    if incentive == 0 {
        println!("No incentive band matches this age (28-29, experienced).");
    } else {
        println!("Annual incentive: N{}", incentive);
    }
}