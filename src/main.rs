mod menu;
mod students;
use menu::display_menu;

fn main() {
    let file_path = "students.json";
    display_menu(file_path);
}
