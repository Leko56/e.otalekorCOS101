struct SalesRecord {
    sn: u32,
    item: &'static str,
    qty: u32,
    amount: f64,
}

fn main() {
    let records = vec![
        SalesRecord { sn: 1, item: "Toshiba", qty: 2, amount: 450_000.00 },
        SalesRecord { sn: 2, item: "Mac",     qty: 1, amount: 1_500_000.00 },
        SalesRecord { sn: 3, item: "HP",      qty: 3, amount: 750_000.00 },
        SalesRecord { sn: 4, item: "Dell",    qty: 3, amount: 2_850_000.00 },
        SalesRecord { sn: 5, item: "Acer",    qty: 1, amount: 250_000.00 },
    ];

    // Print the table
    println!("{:<4}{:<10}{:<6}{:>15}", "S/N", "Item", "Qty", "Amount");
    println!("{}", "-".repeat(35));
    for r in &records {
        println!("{:<4}{:<10}{:<6}{:>15.2}", r.sn, r.item, r.qty, r.amount);
    }

    // Calculate sum
    let total: f64 = records.iter().map(|r| r.amount).sum();

    // Calculate average
    let count = records.len() as f64;
    let average = total / count;

    println!("{}", "-".repeat(35));
    println!("Total Sales Amount   : {:>15.2}", total);
    println!("Average Sales Amount : {:>15.2}", average);

    // Optional: total quantity sold
    let total_qty: u32 = records.iter().map(|r| r.qty).sum();
    println!("Total Quantity Sold  : {}", total_qty);
}