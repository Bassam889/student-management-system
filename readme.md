# Student Management System

This is a simple command-line application written in Rust for managing student data. The application allows you to perform operations such as adding, viewing, searching, updating, and deleting student records.

## Features

- Add a new student
- View all students
- Search for a specific student
- Update student information
- Delete a student
- Save and load student data from a file

## Prerequisites
To run this project, you need:

- Rust (https://www.rust-lang.org/tools/install)

## Installation

1. Clone this repository:
```bash
git clone <repository-url>
cd student-management-system
```

2. Build the project
```bash
cargo build
```

3. Run the project
```bash
cargo run
```

## Usage
The application presents a menu for managing student records. You can choose an
option by entering the corresponding number:

1. **Add a Student:** Add a new student to the system.
2. **View Students:** View a list of all students.
3. **Search for a Student:** Search for a student by name or ID.
4. **Update a Student:** Update the details of an existing student.
5. **Delete a Student:** Remove a student from the system.
6. **Exit:** Exit the application.

## Example
After running the application:

```bash
--- Student Management System ---
1. Add a Student
2. View Students
3. Search for a Student
4. Update a Student
5. Delete a Student
6. Exit
Enter your choice:
```
Follow the prompts to manage student data.

## Project structure
```bash
src/
├── main.rs      # Entry point of the application
├── menu.rs      # Handles the menu and user interaction
└── students.rs  # Contains student-related functionality
```
## Contributing
If you would like to contribute to this project:

1. For the repository
2. Create a new branch:
```bash
git checkout -b feature-name
```
3. Make your changes and commit them:
```bash
git commit -m "Description of changes"
```
4. Push to your fork:
```bash
git push origin feature-name
```
5. Open a pull request

## Contact
For any questions or feedback, please open an issue in the repository.
