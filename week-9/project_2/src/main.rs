// Project II: PAU-SMIS
// Three verbs, in order: read from the vector, display on screen, write to the file.

use std::fs::File;
use std::io::Write;

struct Student {
    name: &'static str,
    matric: &'static str,
    department: &'static str,
    level: u32,
}

// Builds one line of the table so the screen and the file use the same layout.
fn row(name: &str, matric: &str, department: &str, level: &str) -> String {
    format!("{:<18}{:<16}{:<14}{}\n", name, matric, department, level)
}

fn main() {
    // The student records live in a vector.
    let students = vec![
        Student { name: "Oluchi Mordi",   matric: "ACC10211111", department: "Accounting", level: 300 },
        Student { name: "Adams Aliyu",    matric: "ECO10110101", department: "Economics",  level: 100 },
        Student { name: "Shania Bolade",  matric: "CSC10328828", department: "Computer",   level: 200 },
        Student { name: "Adekunle Gold",  matric: "EEE11020202", department: "Electrical", level: 200 },
        Student { name: "Blanca Edemoh",  matric: "MEE10202001", department: "Mechanical", level: 100 },
    ];

    // Verbs 1 and 2: read each student from the vector and display it on screen.
    print!("{}", row("Student Name", "Matric. Number", "Department", "Level"));
    for student in &students {
        print!(
            "{}",
            row(student.name, student.matric, student.department, &student.level.to_string())
        );
    }

    // Verb 3: write to the file.
    // create() opens write-only; any old content in students.txt is destroyed.
    let mut file = File::create("students.txt").expect("could not create students.txt");

    // write_all() sends the header, then one line per student.
    let header = row("Student Name", "Matric. Number", "Department", "Level");
    file.write_all(header.as_bytes()).expect("write failed");

    for student in &students {
        let line = row(student.name, student.matric, student.department, &student.level.to_string());
        file.write_all(line.as_bytes()).expect("write failed");
    }

    println!("\nSaved {} students to students.txt", students.len());
}