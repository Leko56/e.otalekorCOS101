use std::f64::consts::PI;
use std::io::{self, Write};

// Prompt the user and read one number (keeps asking until it is valid).
fn read_number(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<f64>() {
            Ok(value) if value >= 0.0 => return value,
            _ => println!("Please enter a valid non-negative number."),
        }
    }
}

// Trapezium, area = height / 2 * (base1 + base2)
fn trapezium() -> f64 {
    let height = read_number("Enter the height: ");
    let base1 = read_number("Enter base 1: ");
    let base2 = read_number("Enter base 2: ");
    height / 2.0 * (base1 + base2)
}

// Rhombus, area = 1/2 * diagonal1 * diagonal2
fn rhombus() -> f64 {
    let d1 = read_number("Enter diagonal 1: ");
    let d2 = read_number("Enter diagonal 2: ");
    0.5 * d1 * d2
}

// Parallelogram, area = base * altitude
fn parallelogram() -> f64 {
    let base = read_number("Enter the base: ");
    let altitude = read_number("Enter the altitude: ");
    base * altitude
}

// Cube, surface area = 6 * side * side
fn cube() -> f64 {
    let side = read_number("Enter the side length: ");
    6.0 * side * side
}

// Cylinder, volume = pi * radius * radius * height
fn cylinder() -> f64 {
    let radius = read_number("Enter the radius: ");
    let height = read_number("Enter the height: ");
    PI * radius * radius * height
}

fn show_menu() {
    println!("\n===== SHAPE CALCULATOR =====");
    println!("1. Trapezium     (area)");
    println!("2. Rhombus       (area)");
    println!("3. Parallelogram (area)");
    println!("4. Cube          (surface area)");
    println!("5. Cylinder      (volume)");
    println!("0. Quit");
}

fn main() {
    println!("Welcome to the Shape Calculator!");

    loop {
        show_menu();
        print!("Choose a shape (0-5): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => println!("Area of the trapezium = {:.2}", trapezium()),
            "2" => println!("Area of the rhombus = {:.2}", rhombus()),
            "3" => println!("Area of the parallelogram = {:.2}", parallelogram()),
            "4" => println!("Surface area of the cube = {:.2}", cube()),
            "5" => println!("Volume of the cylinder = {:.2}", cylinder()),
            "0" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please pick a number from 0 to 5."),
        }
    }
}