use std::io;

struct ToDoItem {
    id: i32,
    name: String,
    completed: bool,
}

impl ToDoItem {
    fn new(id: i32, name: String) -> ToDoItem {
        ToDoItem { id, name, completed: false }
    }
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
            1 => {
                println!("Enter the new item: ");

                let mut new_item = String::new();
                io::stdin().read_line(&mut new_item).expect("Failed to read line");

                let new_item = ToDoItem::new(
                    (todo_list.len() + 1) as i32, 
                    new_item.trim().to_owned()
                );
                todo_list.push(new_item);
                
                println!("Item added successfully!");
            },
            3 => {
                println!("");
                println!("-------------------------------------");
                for item in &todo_list {
                    println!("[ ] - {}.\t{}\t", item.id, item.name);
                }
                println!("-------------------------------------");
                println!("");
            },
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
