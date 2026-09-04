fn main() {
    let p: f64 = 210_000.00;   // Principal (original value)
    let r: f64 = 5.0;          // Rate of depreciation (%)
    let n: u32 = 3;            // Number of years

    // A = P * [1 - (R/100)]^n
    let a = p * (1.0 - (r / 100.0)).powi(n as i32);

    println!("Original Value (P)      : N{:.2}", p);
    println!("Depreciation Rate (R)    : {}% per annum", r);
    println!("Number of Years (n)      : {}", n);
    println!("Value after {} years (A) : N{:.2}", n, a);

    let total_depreciation = p - a;
    println!("Total Depreciation       : N{:.2}", total_depreciation);
}