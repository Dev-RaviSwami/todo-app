use crate::models::Todo;

pub struct TodoList {
    todos: Vec<Todo>,
    next_id: u32,
}

impl TodoList {
    // Constructor
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    // Add Task
    pub fn add(&mut self, title: String) {
        let todo = Todo::new(self.next_id, title);

        self.todos.push(todo);

        self.next_id += 1;

        println!("Task Added!");
    }

    // Show Tasks
    pub fn show(&self) {
        println!("Your Tasks:");

        if self.todos.is_empty() {
            println!("No tasks found.");
            return;
        }

        for todo in &self.todos {
            println!(
                "{}. {} [{}]",
                todo.id(),
                todo.title(),
                if todo.completed() { "Done" } else { "Pending" }
            );
        }
    }

    // Complete Task
    pub fn complete(&mut self, id: u32) {
        for todo in &mut self.todos {
            if todo.id() == id {
                todo.complete();
                println!("Task Completed!");
                return;
            }
        }

        println!("Task not found!");
    }

    // Delete Task
    pub fn delete(&mut self, id: u32) {
        let index = self.todos.iter().position(|todo| todo.id() == id);

        match index {
            Some(index) => {
                self.todos.remove(index);
                println!("Task Deleted!");
            }
            None => {
                println!("Task not found!");
            }
        }
    }
}
