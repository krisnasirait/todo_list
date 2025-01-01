use std::io;

//struct task to store the Task data
// struct Task {
//     id: Int,
//     description: String,
//     is_completed: bool
// }

fn main() {
    showMenu();
}

//fn to add task
pub fn add() {
    print!("\x1B[2J\x1B[1;1H");
    println!("======ADD YOUR TASK======");
}

//fn to edit task
pub fn edit() {
    print!("\x1B[2J\x1B[1;1H");
    println!("======EDIT YOUR TASK======");

}

//fn to delete task
pub fn delete() {
    print!("\x1B[2J\x1B[1;1H");
    println!("======DELETE YOUR TASK======");

}

//fn to show main menu
pub fn showMenu() {

    let mut option: u8 = 0;

    println!("======SIMPLE TO DO LIST======");
    println!("Build in RUST...by @krisnasirait");
    println!("1. Add Task");
    println!("2. Edit Task");
    println!("3. Delete Task");

    let mut option_choosen = String::new();

    io::stdin().read_line(&mut option_choosen).expect("Failed to read option choosen");
    if let Ok(val) = option_choosen.trim().parse::<u8>() {
        option = val;
        match option {
            1 => add(),
            2 => edit(),
            3 => delete(),
            _ => println!("Invalid"),
        }
    } else {
        println!("Please enter valid number");
    }
}
