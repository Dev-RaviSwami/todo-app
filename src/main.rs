mod models;
mod todo_list;

use std::io;
use todo_list::TodoList;

fn main() {
    let mut app = TodoList::new();

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
                println!("Enter Task:");

                let mut title = String::new();

                io::stdin().read_line(&mut title).unwrap();

                app.add(title.trim().to_string());
            }

            "2" => {
                app.show();
            }

            "3" => {
                println!("Enter task id:");

                let mut id = String::new();

                io::stdin().read_line(&mut id).unwrap();

                let id: u32 = id.trim().parse().unwrap();

                app.complete(id);
            }

            "4" => {
                println!("Enter task id:");

                let mut id = String::new();

                io::stdin().read_line(&mut id).unwrap();

                let id: u32 = id.trim().parse().unwrap();

                app.delete(id);
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
