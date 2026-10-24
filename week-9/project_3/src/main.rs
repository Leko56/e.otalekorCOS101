// Project III: The EFCC Merge
// Three separate datasets (commissioner, ministry, zone) are merged by index
// into one file, one line per minister. Row 1 of each dataset is the same person.

use std::fs::File;
use std::io::Write;

// Builds one line of the merged table.
fn row(commissioner: &str, ministry: &str, zone: &str) -> String {
    format!("{:<28}{:<18}{}\n", commissioner, ministry, zone)
}

fn main() {
    // Dataset 1, Dataset 2 and Dataset 3 from the cloud backup.
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];
    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // create() opens write-only; any old content in merged_commissioners.txt is destroyed.
    let mut file = File::create("merged_commissioners.txt")
        .expect("could not create merged_commissioners.txt");

    // Header line.
    let header = row("Commissioner", "Ministry", "Zone");
    print!("{}", header);
    file.write_all(header.as_bytes()).expect("write failed");

    // Merge by index: the same position in each vector is the same person.
    for i in 0..commissioners.len() {
        let line = row(commissioners[i], ministries[i], zones[i]);
        print!("{}", line);
        file.write_all(line.as_bytes()).expect("write failed");
    }

    println!("\nMerged {} records into merged_commissioners.txt", commissioners.len());
}