use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::{process, usize};

const TASKS_FILE: &str = "todo_list.txt";
fn main() {
    //let args: Vec<String> = env::args().collect();
    
    loop {

        println!("Enter a command (add, list, complete, delete, exit):");
        let mut input = String::new();
        std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

        let args: Vec<String> = input.trim().split_whitespace().take(2).map(String::from).collect();

        let task = Task::new(&args).unwrap_or_else(|err| {
            eprintln!("Problem {err}");
            process::exit(1);
        });

        match task.command.as_str() {
            "add" => {
                if let Err(e) = task.add() {
                    eprintln!(" {e}");
                    process::exit(1);
                }
            }
            "list" => {
                if let Err(e) = task.list() {
                    eprintln!("{e}");
                    process::exit(1);
                }
            }
            "complete" => {
                if let Err(e) = task.complete() {
                    eprintln!("{e}");
                    process::exit(1);
                }
            }
            "delete" => {
                if let Err(e) = task.delete() {
                    eprintln!("{e}");
                    process::exit(1);
                }
            }
            "exit" => {
                println!("Exiting");
                break;
            
            }
            _ => println!("Unknown command. Use add, list, complete, delete, exit."),
        } 
    }
}

struct Task {
    command: String,
    query: String,
}

impl Task {
    fn new(args: &[String]) -> Result<Task, &'static str> {
        if args.len() < 1 {
            return Err("not enough arguments. Usage: <command> <query>");
        }

        let command = args[0].clone();

        let query = if args.len() > 1 {
            args[1].clone()
        } else {
            String::new()
        };

        Ok(Task { command, query })
    }
//adding a task with [ ]
    fn add(&self) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(TASKS_FILE)?;
        let mut writer = io::BufWriter::new(file);
        writeln!(writer, "[ ] {}", self.query)?;
        println!("Adding task {}", self.query);
        self.list()?;
        Ok(())
    }

//listing a task
    fn list(&self) -> io::Result<()> {
        let file = OpenOptions::new().read(true).open(TASKS_FILE)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
        println!("Listing");
        Ok(())
    }

// marking a task complete [x]
    fn complete(&self) -> io::Result<()> {

        if !Path::new(TASKS_FILE).exists() {
            println!("No tasks found. Add some tasks!!");
            return Ok(());
        }

        let task_id: usize = match self.query.parse() {
            Ok(id) => id,
            Err(_) => {
                println!("Invalid task ID. Provide a number");
                return Ok(());
            }
        };

        let file = File::open(TASKS_FILE)?;

        let reader = BufReader::new(file);
        let mut tasks: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        if task_id == 0 || task_id > tasks.len() {
            println!("Task ID not in range. Enter valid task id!!");
            return Ok(());
        }

        let task_index = task_id - 1;
        let task = &tasks[task_index];

        if task.starts_with("[x]") {
            println!("Task already completed.");
            return Ok(());
        }

        if task.starts_with("[ ]") {
            tasks[task_index] = task.replacen("[ ]", "[x]", 1);
        } else {
            tasks[task_index] = format!("[x] {}", task);
        }

        let mut file = File::create(TASKS_FILE)?;

        for task in &tasks {
            writeln!(file, "{}", task)?;
        }

        println!("Task {} completed", tasks[task_index]);

        self.list()?;
        Ok(())
    }

//deletes task based on number
    fn delete(&self) -> io::Result<()> {
        let file = File::open(TASKS_FILE)?;
        let reader = BufReader::new(file);
        let mut tasks: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        let mut task_index = None;

        for (i, task) in tasks.iter().enumerate() {
            if task.contains(&self.query) {
                task_index = Some(i);
                break;
            }
        }

        if let Some(index) = task_index {
            let removed_task = tasks.remove(index);
            println!("The task {removed_task} removed");
        } else {
            println!("No task containing {} was found", self.query);
        }
        let mut file = File::create(TASKS_FILE)?;

        for task in &tasks {
            writeln!(file, "{}", task)?;
        }

        println!("Deleting");
        self.list()?;
        Ok(())
    }
}
