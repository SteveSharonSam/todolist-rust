# Rust CLI To-Do List

A simple and efficient command-line to-do list application written in Rust. A very simple application which was written to just learn rust.

## Features

* **Add:** Add a new task to your list.
* **List:** Display all current tasks with their status (`[ ]` for pending, `[x]` for complete).
* **Complete:** Mark a task as complete using its number.
* **Delete:** Remove a task from the list by its content.
* **Persistent Storage:** Your tasks are automatically saved to `todo_list.txt`.

## How to Use

1.  Clone the repository and navigate into the directory.
2.  Build the project using Cargo:
    ```sh
    cargo build --release
    ```
3.  Run the application from your terminal:
    ```sh
    ./target/release/your_binary_name
    ```
    *(Replace `your_binary_name` with the name of your executable, which is usually the name of your project folder).*

4.  Enter commands at the prompt:
    * `add <Your task description>`
    * `list`
    * `complete <task_number>`
    * `delete <task_description>`
    * `exit`
