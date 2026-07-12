mod models;
mod todo;
use std::io;

use models::Todo;
use todo::{add_task, complete_task, delete_task, show_tasks};
fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    let mut counter = 0;
    loop {
        println!("\n===== TODO APP =====");
        println!("1. Add Task");
        println!("2. Show Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let mut title = String::new();
                println!("Enter Task: ");
                io::stdin().read_line(&mut title).unwrap();

                add_task(&mut todos, &mut counter, title);
            }

            "2" => {
                show_tasks(&todos);
            }
            "3" => {
                println!("Enter task id: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();

                let id: u32 = id.trim().parse().unwrap();

                complete_task(&mut todos, id)
            }
            "4" => {
                println!("Enter task id: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();

                let id: u32 = id.trim().parse().unwrap();
                delete_task(&mut todos, id);
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}
