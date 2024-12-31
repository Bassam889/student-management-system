use crate::students::{add_student, delete_student, search_student, update_student, view_students};
use std::io::{self, Write};
pub fn display_menu(file_path: &str) {
    loop {
        println!("\n--- Student Management System ---");
        println!("1. Add a Student");
        println!("2. View Students");
        println!("3. Search for a Student");
        println!("4. Update a Student");
        println!("5. Delete a Student");
        println!("6. Exit");
        println!("Enter your choice");

        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim().parse::<u32>() {
            Ok(1) => {
                add_student(file_path);
            }
            Ok(2) => {
                view_students(file_path);
            }
            Ok(3) => {
                search_student(file_path);
            }
            Ok(4) => {
                update_student(file_path);
            }
            Ok(5) => {
                delete_student(file_path);
            }
            Ok(6) => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice, try again.");
            }
        }
    }
}
