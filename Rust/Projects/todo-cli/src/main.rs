use std::fs;
use std::io::{Write, BufRead, BufReader};
use std::usize;
fn main() {
    println!("Welcome to Rust todo!");

    let mut todos = load_todos().unwrap_or_else(|_| Vec::new());

    loop {
        println!("\n1. Add Todo\n2. List Todos\n3. Remove Todo\n4. Quit");
        let choice = read_line();


        match choice.trim() {
            "1" => add_todo(&mut todos),
            "2" => list_todos(&todos),
            "3" => remove_todo(&mut todos),
            "4" => break,
            _ => println!("Invalid option")
        }
    }

    match save_todos(&todos) {
        Ok(_) => println!("Todos saved"),
        Err(e) => println!("Failed to save todos: {}", e)
    }
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_owned()
}

fn load_todos() -> std::io::Result<Vec<String>> {
    let file  = fs::File::open("todos.txt")?;
    let reader = BufReader::new(file);
    Ok(reader.lines().collect::<Result<Vec<_>, _>>()?)
}

fn save_todos(todos: &[String]) -> std::io::Result<()> {
    let mut file = fs::File::create("todos.txt")?;
    for todo in todos {
        writeln!(file, "{}", todo)?;
    }
    Ok(())
}

fn add_todo(todos: &mut Vec<String>) {
    println!("Add todo: ");
    let todo = read_line();
    if !todo.is_empty() {
        todos.push(todo);
        println!("Todo added");
    }
}

fn list_todos(todos: &[String]) {
    if todos.is_empty() {
        println!("No todos yet");
    } else {
        for (index, todo) in todos.iter().enumerate() {
            println!("{}. {}", index + 1, todo);
        }
    }
}

fn remove_todo(todos: &mut Vec<String>) {
    list_todos(todos);
    if !todos.is_empty() {
        println!("Enter the number of the todo to remove");
        if let Ok(num) = read_line().parse::<usize>() {
            if num > 0 && num <= todos.len() {
                todos.remove(num - 1);
                println!("Todo removed");
            } else {
                println!("Invalid number");
            }
        }
    }
}


