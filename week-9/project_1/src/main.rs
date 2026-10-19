use std::io::Write;

fn main() {

    let header = "Lager          Stout          Non-Alcoholic\n";
    let row1 = "33 Export      Legend         Maltina\n";
    let row2 = "Desperados     Turbo King     Amstel Malta\n";
    let row3 = "Goldberg       Williams       Malta Gold\n";
    let row4 = "Gulder                        Fayrouz\n";
    let row5 = "Heineken\n";
    let row6 = "Star\n";

    let mut file = std::fs::File::create("drinks.txt").expect("create failed");
    file.write_all(header.as_bytes()).expect("write failed");
    file.write_all(row1.as_bytes()).expect("write failed");
    file.write_all(row2.as_bytes()).expect("write failed");
    file.write_all(row3.as_bytes()).expect("write failed");
    file.write_all(row4.as_bytes()).expect("write failed");
    file.write_all(row5.as_bytes()).expect("write failed");
    file.write_all(row6.as_bytes()).expect("write failed");
    println!("\nData written to file.");

}