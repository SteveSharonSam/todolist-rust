use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write, Result};
use std::path::Path;

const TASKS_FILE: &str = "todo_list.txt";

enum Command {
    Add(String),
    List,
    Complete(usize),
    Delete(String),
    Exit,
    Unknown(String), 
}

fn main() -> Result<()> {
    loop {
        println!("Enter a command (add, list, complete, delete, exit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?; 

        //parsing input
        let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
        let command_name = parts.get(0).unwrap_or(&"");
        let query = parts.get(1).unwrap_or(&"").to_string();

        let command = match *command_name {
            "add" => Command::Add(query),
            "list" => Command::List,
            "complete" => match query.parse::<usize>() {
                Ok(id) => Command::Complete(id),
                Err(_) => {
                    println!("Invalid task ID. Please provide a number.");
                    continue; 
                }
            },
            "delete" => Command::Delete(query),
            "exit" => Command::Exit,
            _ => Command::Unknown(command_name.to_string()),
        };

        let result = match command {
            Command::Add(task) => add_task(&task),
            Command::List => list_tasks(),
            Command::Complete(id) => complete_task(id),
            Command::Delete(task_query) => delete_task(&task_query),
            Command::Exit => {
                println!("Exiting");
                break; // Exit the loop
            }
            Command::Unknown(cmd) => {
                println!("Unknown command: '{}'. Use add, list, complete, delete, exit.", cmd);
                Ok(()) // Continue the loop
            }
        };

        if let Err(e) = result {
            eprintln!("An error occurred: {}", e);
        }
    }
    Ok(())
}


fn add_task(query: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(TASKS_FILE)?; 
    writeln!(file, "[ ] {}", query)?;
    println!("Adding task: {}", query);
    list_tasks()?;
    Ok(())
}

fn list_tasks() -> io::Result<()> {
    if !Path::new(TASKS_FILE).exists() {
        println!("No tasks found. Add some tasks!");
        return Ok(());
    }
    let file = File::open(TASKS_FILE)?;
    let reader = BufReader::new(file);
    println!("--- TODO LIST ---");
    for (index, line) in reader.lines().enumerate() {
        println!("{}: {}", index + 1, line?);
    }
    println!("-----------------");
    Ok(())
}

fn complete_task(task_id: usize) -> io::Result<()> {
    let mut tasks = read_tasks_from_file()?;

    if task_id == 0 || task_id > tasks.len() {
        println!("Task ID not in range. Please enter a valid task ID.");
        return Ok(());
    }

    let task_index = task_id - 1;
    if tasks[task_index].starts_with("[x]") {
        println!("Task already completed.");
    } else {
        tasks[task_index] = tasks[task_index].replacen("[ ]", "[x]", 1);
        println!("Task {} completed.", task_id);
    }

    write_tasks_to_file(&tasks)?;
    list_tasks()?;
    Ok(())
}

fn delete_task(query: &str) -> io::Result<()> {
    let mut tasks = read_tasks_from_file()?;
    let original_len = tasks.len();

    tasks.retain(|task| !task.contains(query));

    if tasks.len() < original_len {
        println!("Task containing '{}' was removed.", query);
        write_tasks_to_file(&tasks)?;
        list_tasks()?;
    } else {
        println!("No task containing '{}' was found.", query);
    }

    Ok(())
}


fn read_tasks_from_file() -> io::Result<Vec<String>> {
    if !Path::new(TASKS_FILE).exists() {
        return Ok(Vec::new()); 
    }
    let file = File::open(TASKS_FILE)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn write_tasks_to_file(tasks: &[String]) -> io::Result<()> {
    let mut file = File::create(TASKS_FILE)?;
    for task in tasks {
        writeln!(file, "{}", task)?;
    }
    Ok(())
}