mod db;

use db::{init_db, add_todo, list_todos, mark_done, delete_todo};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Todo List with DB")]
#[command(about = "A simple CLI Todo List using SQLite")]
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
    Done { id: i32 },

    /// Delete a todo
    Delete { id: i32 },
}

fn main() {
    let conn = init_db().expect("Failed to initialize database");
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title } => {
            add_todo(&conn, &title).expect("Failed to add todo");
            println!("Added: {}", title);
        }
        Commands::List => {
            let todos = list_todos(&conn).expect("Failed to fetch todos");
            for todo in todos {
                println!(
                    "{}. [{}] - {}",
                    todo.id,
                    if todo.completed { "✔" } else { " " },
                    todo.title
                );
            }
        }
        Commands::Done { id } => {
            mark_done(&conn, id).expect("Failed to mark todo as done");
            println!("Marked as done: {}", id);
        }
        Commands::Delete { id } => {
            delete_todo(&conn, id).expect("Failed to delete todo");
            println!("Deleted todo {}", id);
        }
    }
}
