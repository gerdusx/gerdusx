use std::io;

struct ToDoItem {
    id: i32,
    name: String,
    completed: char,
}

fn main() {
    let mut todo_list: Vec<ToDoItem> = Vec::new();

    loop {
        println!("");
        println!("Welcome to your ToDo App!!");
        println!("1. Add a new item");
        println!("2. Mark an item as completed");
        println!("3. List all items");
        println!("4. Exit");
    
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line");

        let option = option.trim().parse::<u32>().expect("Please enter a number");

        match option {
            4 => {
                println!("Goodbye!");
                break;
            },
            _ => {
                println!("Invalid option");
            }
        }
    }
}
