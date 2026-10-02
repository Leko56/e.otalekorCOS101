use std::io;

// Week 6 Class Project: The Restaurant Menu
// Reads a food type and a quantity, computes the total charge,
// and gives a 5% discount when the total is greater than N10,000.

fn read_line_trimmed() -> String {
    let mut input = String::new();                 // Practice 2: String::new()
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()                       // Practice 5: trim()
}

fn main() {
    // Display the menu (string literals, Practice 1)
    println!("=======================================");
    println!("          RESTAURANT MENU");
    println!("=======================================");
    println!(" P  Poundo Yam / Edinkaiko Soup  N3,200");
    println!(" F  Fried Rice & Chicken         N3,000");
    println!(" A  Amala & Ewedu Soup           N2,500");
    println!(" E  Eba & Egusi Soup             N2,000");
    println!(" W  White Rice & Stew            N2,500");
    println!("=======================================");

    // Read the food type
    println!("Enter the food code (P, F, A, E or W):");
    let code = read_line_trimmed().to_uppercase();

    // Decide on the letter: price (in naira) and name for each choice
    let (name, price): (&str, u32) = match code.as_str() {
        "P" => ("Poundo Yam / Edinkaiko Soup", 3200),
        "F" => ("Fried Rice & Chicken", 3000),
        "A" => ("Amala & Ewedu Soup", 2500),
        "E" => ("Eba & Egusi Soup", 2000),
        "W" => ("White Rice & Stew", 2500),
        _ => {
            println!("Invalid food code: '{}'. Please run the program again.", code);
            return;
        }
    };

    // Read the quantity
    println!("Enter the quantity:");
    let quantity: u32 = match read_line_trimmed().parse() {
        Ok(q) if q > 0 => q,                       // relational operator: q > 0
        _ => {
            println!("Invalid quantity. Please enter a whole number greater than 0.");
            return;
        }
    };

    // Arithmetic for the total (Practice 8)
    let total = price * quantity;

    // One last if for the discount: 5% when total is greater than N10,000
    let discount = if total > 10000 {
        total * 5 / 100
    } else {
        0
    };
    let amount_to_pay = total - discount;

    // Build the receipt with format! (Practice 7)
    let receipt = format!(
        "\n---------------- RECEIPT ----------------\n\
         Item     : {}\n\
         Price    : N{}\n\
         Quantity : {}\n\
         Total    : N{}\n\
         Discount : N{}\n\
         Pay      : N{}\n\
         -----------------------------------------",
        name, price, quantity, total, discount, amount_to_pay
    );
    println!("{}", receipt);

    if discount > 0 {
        println!("A 5% discount was applied because your total is above N10,000.");
    }
}