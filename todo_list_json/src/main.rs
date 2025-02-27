mod todo;

use std::{fs};
use serde_json;
use todo::Todo;
use clap::{Parser, Subcommand};

const FILE_PATH: &str = "todo.json";

#[derive(Parser)]
#[command(name = "Todo List")]
#[command(about = "Simple CLI Todo List in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo
    Add { title: String },
    
    /// List all todos
    List,
    
    /// Mark a todo as done
    Done { id: usize },
    
    /// Delete a todo
    Delete { id: usize },
}

fn load_todos() -> Vec<Todo> {
    if let Ok(data) = fs::read_to_string(FILE_PATH) {
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

fn save_todos(todos: &Vec<Todo>) {
    let data = serde_json::to_string_pretty(todos).expect("Failed to serialize data");
    fs::write(FILE_PATH, data).expect("Failed to save data");
}

fn main() {
    let cli = Cli::parse();
    let mut todos = load_todos();

    match cli.command {
        Commands::Add { title } => {
            let id = todos.len() + 1;
            todos.push(Todo::new(id, &title));
            save_todos(&todos);
            println!("Added: {}", title);
        }
        Commands::List => {
            for todo in &todos {
                println!(
                    "{}. [{}] - {}",
                    todo.id,
                    if todo.completed { "✔" } else { " " },
                    todo.title
                );
            }
        }
        Commands::Done { id } => {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.completed = true;
            }
            save_todos(&todos);
            println!("Marked as done: {}", id);
        }
        Commands::Delete { id } => {
            todos.retain(|t| t.id != id);
            save_todos(&todos);
            println!("Deleted todo {}", id);
        }
    }
}
