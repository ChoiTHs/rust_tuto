use rusqlite::{params, Connection, Result};

pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("todo.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    )?;
    Ok(conn)
}

pub fn add_todo(conn: &Connection, title: &str) -> Result<()> {
    conn.execute("INSERT INTO todos (title, completed) VALUES (?1, ?2)", params![title, false])?;
    Ok(())
}

pub fn list_todos(conn: &Connection) -> Result<Vec<Todo>> {
    let mut stmt = conn.prepare("SELECT id, title, completed FROM todos")?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    let mut todos = Vec::new();
    for todo in todo_iter {
        todos.push(todo?);
    }
    Ok(todos)
}

pub fn mark_done(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("UPDATE todos SET completed = 1 WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn delete_todo(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM todos WHERE id = ?1", params![id])?;
    Ok(())
}
