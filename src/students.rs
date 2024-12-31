use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    id: u32,
    name: String,
    age: u8,
    grade: String,
    gpa: f32,
}

impl Student {
    pub fn new(id: u32, name: String, age: u8, grade: String, gpa: f32) -> Self {
        Self {
            id,
            name,
            age,
            grade,
            gpa,
        }
    }
}

pub fn add_student(file_path: &str) {
    // Load existing students from the file
    let mut students = load_students_from_file(file_path);

    // Gather new student details
    println!("Enter Student ID: ");
    let id = read_input::<u32>();

    // Check for duplicate ID
    if students.contains_key(&id) {
        println!("A student with this ID already exists. Please try again.");
        return;
    }

    println!("Enter Student Name: ");
    let name = read_input::<String>();

    println!("Enter Student Age: ");
    let age = read_input::<u8>();

    println!("Enter Student Grade: ");
    let grade = read_input::<String>();

    println!("Enter Student GPA: ");
    let gpa = read_input::<f32>();

    // Create and add the new student
    let student = Student::new(id, name, age, grade, gpa);
    students.insert(id, student);

    // Save the updated list of students back to the file
    save_students_to_file(&students, file_path);

    println!("Student added successfully!");
}

pub fn view_students(file_path: &str) {
    // Load existing students from the file
    let students = load_students_from_file(file_path);

    // Check if the list is empty
    if students.is_empty() {
        println!("No students found.");
        return;
    }

    // Iterate through each student and print their details
    for student in students.values() {
        println!(
            "ID: {}, Name: {}, Age: {}, Grade: {}, GPA: {:.2}",
            student.id, student.name, student.age, student.grade, student.gpa
        );
    }
}

pub fn search_student(file_path: &str) {
    // Load students from the file
    let students = load_students_from_file(file_path);

    // Check if the list is empty
    if students.is_empty() {
        println!("No students found. The file is empty.");
        return;
    }

    // Prompt for the student ID to search
    println!("Enter Student ID to search: ");
    let id = read_input::<u32>();

    // Search for the student
    match students.get(&id) {
        Some(student) => {
            println!(
                "ID: {}, Name: {}, Age: {}, Grade: {}, GPA: {:.2}",
                student.id, student.name, student.age, student.grade, student.gpa
            );
        }
        None => println!("Student not found."),
    }
}

pub fn update_student(file_path: &str) {
    // Load students from the file
    let mut students = load_students_from_file(file_path);

    // Check if the list is empty
    if students.is_empty() {
        println!("No students found. The file is empty.");
        return;
    }

    // Prompt for the Student ID to update
    println!("Enter Student ID to update: ");
    let id = read_input::<u32>();

    // Check if the student exists
    if let Some(student) = students.get_mut(&id) {
        // Display current details
        println!("Enter new name (or press Enter to skip):");
        let name = read_optional_input::<String>();
        if let Some(name) = name {
            student.name = name;
        }

        // Allow user to update fields
        println!("Enter new age (or press Enter to skip):");
        let age = read_optional_input::<u8>();
        if let Some(age) = age {
            student.age = age;
        }

        println!("Enter new grade (or press Enter to skip):");
        let grade = read_optional_input::<String>();
        if let Some(grade) = grade {
            student.grade = grade;
        }

        println!("Enter new GPA (or press Enter to skip):");
        let gpa = read_optional_input::<f32>();
        if let Some(gpa) = gpa {
            student.gpa = gpa;
        }

        // Save updated list back to the file
        save_students_to_file(&students, file_path);

        println!("Student updated successfully!");
    } else {
        println!("Student not found.");
    }
}

pub fn delete_student(file_path: &str) {
    // Load students from the file
    let mut students = load_students_from_file(file_path);

    // Check if the list is empty
    if students.is_empty() {
        println!("No students found. The file is empty.");
        return;
    }

    // Prompt for the student ID to delete
    println!("Enter Student ID to delete:");
    let id = read_input::<u32>();

    // Check if the student exists
    if students.remove(&id).is_some() {
        // Save the updated list back to the file
        save_students_to_file(&students, file_path);
        println!("Student deleted successfully");
    } else {
        println!("Student not found.");
    }
}

pub fn read_input<T: std::str::FromStr>() -> T {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().ok().expect("Invalid input, try again")
}

pub fn read_optional_input<T: std::str::FromStr>() -> Option<T> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else {
        trimmed.parse().ok()
    }
}

pub fn save_students_to_file(students: &HashMap<u32, Student>, file_path: &str) {
    match File::create(file_path) {
        Ok(mut file) => {
            let json = serde_json::to_string_pretty(students).unwrap();
            if let Err(error) = file.write_all(json.as_bytes()) {
                eprintln!("Failed to write to file: {}", error);
            } else {
                println!("Students saved successfully to {}", file_path);
            }
        }
        Err(error) => eprintln!("Failed to create file: {}", error),
    }
}

pub fn load_students_from_file(file_path: &str) -> HashMap<u32, Student> {
    match fs::read_to_string(file_path) {
        Ok(data) => match serde_json::from_str::<HashMap<u32, Student>>(&data) {
            Ok(students) => {
                println!("Students loaded successfully from {}", file_path);
                students
            }
            Err(error) => {
                eprintln!("Failed to parse JSON: {}", error);
                HashMap::new()
            }
        },
        Err(_) => {
            println!("File not found, starting with an empty list of students.");
            HashMap::new()
        }
    }
}
