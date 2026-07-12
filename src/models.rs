pub struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

impl Todo {
    // Constructor
    pub fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }

    // Mark task as completed
    pub fn complete(&mut self) {
        self.completed = true;
    }

    // Rename task
    pub fn rename(&mut self, title: String) {
        self.title = title;
    }

    // Getters
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn status(&self) -> &str {
        if self.completed { "Done" } else { "Pending" }
    }
}
