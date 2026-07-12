use crate::models::Todo;

pub fn add_task(todos: &mut Vec<Todo>, counter: &mut u32, title: String) {
    *counter += 1;
    let todo = Todo {
        id: *counter,
        title: title.trim().to_string(),
        completed: false,
    };

    todos.push(todo);
    println!("Task Added!");
}

pub fn show_tasks(todos: &Vec<Todo>) {
    println!("Your Tasks: ");

    for todo in todos {
        println!(
            "{}. {} [{}]",
            todo.id,
            todo.title,
            if todo.completed { "Done" } else { "Pending" }
        );
    }
}

pub fn complete_task(todos: &mut Vec<Todo>, id: u32) {
    for todo in todos {
        if todo.id == id {
            todo.completed = true;
            println!("Task Completed!");
        }
    }
}

pub fn delete_task(todos: &mut Vec<Todo>, id: u32) {
    let index = todos.iter().position(|x| x.id == id);
    match index {
        Some(index) => {
            todos.remove(index);
            println!("Task Deleted!");
        }
        None => {
            println!("Task not found!");
        }
    }
}
