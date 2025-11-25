use std::io;
use std::io::Write;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Tasks {
    id: i32,
    task: String
}

fn main() {
    loop {
        println!("RustyTodo");

        println!("1. Add a new task");
        println!("2. Read tasks");
        println!("3. Delete a task");
        print!("What would you like to do: ");
        io::stdout().flush().unwrap();

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .unwrap();

        let number: i32 = match number.trim().parse() {
            Ok(number_check) => number_check,
            Err(_) => {
                println!("\nInvalid number");
                continue;
            }
        };

        if number == 1 {
            print!("Please enter something todo: ");
            io::stdout().flush().unwrap();

            let mut input_variable = String::new();

            io::stdin()
                .read_line(&mut input_variable)
                .unwrap();

            add_to_db(input_variable.trim());

        } else if number == 2 {
            println!("\nOn your todo list");
            println!("-----------------");
            read_from_db();
        } else if number == 3 {
            print!("\nEnter the ID of the task you want to delete: ");
            io::stdout().flush().unwrap();

            let mut task_id = String::new();

            io::stdin()
                .read_line(&mut task_id)
                .unwrap();

            let task_id: i32 = match task_id.trim().parse() {
                Ok(task_id_check) => task_id_check,
                Err(_) => {
                    println!("Invalid number");
                    continue;
                }
            };

            delete_from_db(task_id);

        } else {
            println!("Closing program");
            break;
        }
    }


}


/// Stores the task in the DB
fn add_to_db(todo: &str) {
    // Open the database from file
    let conn = Connection::open("./sqlite/todo_list.db");

    // Add the task to DB
    conn.unwrap().execute("INSERT INTO todo (task) VALUES (?1)", params![todo]).expect("TODO: panic message");


}

/// Collects the data stored in the DB and prints it out in a list
fn read_from_db() {
    // Open the database from file
    let conn = Connection::open("./sqlite/todo_list.db").unwrap();

    // Read task(s) from DB
    let mut task_list = conn.prepare("SELECT * FROM todo").unwrap();
    let task_list_iterator = task_list.query_map([], |row| {
        Ok(
            Tasks {
                id: row.get(0)?,
                task: row.get(1)?
            }
        )
    }).unwrap();

    let collect_tasks: Vec<Tasks> = task_list_iterator.collect::<Result<Vec<_>, _>>().unwrap();

    for task in collect_tasks {
        println!("{}: {}", task.id, task.task)
    }

}

/// Removes the selected task from the DB
fn delete_from_db(task_number: i32) {
    // Open the database
    let conn = Connection::open("./sqlite/todo_list.db");

    // Remove task from DB
    conn.unwrap().execute("DELETE FROM todo WHERE id = ?1", params![task_number]).expect("TODO: panic message");
}
