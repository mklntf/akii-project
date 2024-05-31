use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Task {
    id: u32,
    name: String,
}

impl Task {
    pub fn new(id: u32, name: String) -> Self {
        Task { id, name }
    }
}

pub struct Service {
    tasks: Mutex<Vec<Task>>,
    count: Mutex<u32>,
}

impl Service {
    pub fn new() -> Self {
        Service {
            tasks: Mutex::new(vec![]),
            count: Mutex::new(0),
        }
    }

    pub fn list_tasks(&self) -> Vec<Task> {
        let tasks = self.tasks.lock().unwrap();
        tasks.clone()
    }

    pub fn create_task(&self, name: String) -> Task {
        let mut tasks = self.tasks.lock().unwrap();
        let mut count = self.count.lock().unwrap();

        let id = *count;
        *count += 1;

        let task = Task::new(id, name);
        tasks.push(task.clone());

        task
    }

    pub fn get_task(&self, id: u32) -> Option<Task> {
        let tasks = self.tasks.lock().unwrap();
        tasks.iter().find(|task| task.id == id).cloned()
    }

    pub fn delete_task(&self, id: u32) -> Option<Task> {
        let mut tasks = self.tasks.lock().unwrap();
        let task = tasks.iter().position(|task| task.id == id);

        match task {
            Some(index) => Some(tasks.remove(index)),
            None => None,
        }
    }
}
