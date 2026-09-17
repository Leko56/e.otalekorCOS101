use std::io;

fn main() {
    // 1. Read the three coefficients: a, b, c
    println!("Enter a:");
    let mut a_input = String::new();
    io::stdin()
        .read_line(&mut a_input)
        .expect("Failed to read input");
    let a: f64 = a_input.trim().parse().expect("Input not a number");

    println!("Enter b:");
    let mut b_input = String::new();
    io::stdin()
        .read_line(&mut b_input)
        .expect("Failed to read input");
    let b: f64 = b_input.trim().parse().expect("Input not a number");

    println!("Enter c:");
    let mut c_input = String::new();
    io::stdin()
        .read_line(&mut c_input)
        .expect("Failed to read input");
    let c: f64 = c_input.trim().parse().expect("Input not a number");

    // 2. The discriminant decides everything
    let d: f64 = b * b - 4.0 * a * c;

    // 3. Branch on the sign of d with an else if ladder
    if d > 0.0 {
        // Two distinct real roots
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {} and {}", root1, root2);
    } else if d == 0.0 {
        // Exactly one real root (a repeated root)
        let root = -b / (2.0 * a);
        println!("One real root: {}", root);
    } else {
        // d < 0.0: no real roots
        println!("No real roots (discriminant is negative).");
    }
}