



fn main() {
    //I want to load a list of todos from a file, list of strings, and load them into a vector
    //I want to be able to add a todo to the list
    //I want to be able to remove a todo from the list
    //I want to be able to save the list of todos to a file
    //I want to display the list of todos to the user

    let mut todos = load_todos();

    loop {
        display_todos(&todos);

        let input = read_input();

        if input == 1 {
            add_todo(&mut todos);
        } else if input == 2 {
            println!("Which todo would you like to remove?");
            for (index, todo) in todos.iter().enumerate() {
                println!("{}. {}", index + 1, todo);
            }
            let mut index = String::new();
            std::io::stdin().read_line(&mut index).unwrap();
            let index: usize = index.trim().parse().unwrap();
            todos.remove(index - 1);
        }else if input == 3 {
            save_todos(&todos);
            break;
        } else {
            println!("Invalid input");
        }
    }
}

fn add_todo(todos: &mut Vec<String>) {
    println!("What todo would you like to add?");
    let mut new_todo = String::new();
    std::io::stdin().read_line(&mut new_todo).unwrap();
    todos.push(new_todo.trim().to_string());
}

//function to load todos from a file
fn load_todos() -> Vec<String> {
    let contents = std::fs::read_to_string("todos.txt").unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

//functiob to display todos
fn display_todos(todos: &Vec<String>) {
    //clear console
    print!("\x1B[2J\x1B[1;1H");
    println!("------------------------------");
    println!("Todos:");
    if todos.is_empty() {
        println!("You have no todos!");
    } else {
        //print all todos with index + 1 at the start
        for (index, todo) in todos.iter().enumerate() {
            println!("{}. {}", index + 1, todo);
        }
    }

    println!("------------------------------");
    //print option in one line, like (1) Add a todo...
    println!("1. Add a todo\n2. Remove a todo\n3. Quit");
    println!("");
    println!("Enter your choice: ");
}

//function to read user input
fn read_input() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: u32 = input.trim().parse().unwrap();
    input
}

//function to save todos to a file
fn save_todos(todos: &Vec<String>) {
    let mut todo_str = String::new();
    for todo in todos.iter() {
        todo_str.push_str(todo);
        todo_str.push_str("\n");
    }
    std::fs::write("todos.txt", todo_str).unwrap();
}

//function to remove a todo
fn remove_todo() {
    
}