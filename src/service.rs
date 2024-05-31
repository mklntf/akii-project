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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_new() {
        let task = Task::new(0, "Example Task".to_string());

        assert_eq!(task.id, 0);
        assert_eq!(task.name, "Example Task");
    }

    #[test]
    fn test_service_new() {
        let service = Service::new();

        let tasks = service.list_tasks();
        assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn test_service_create_task() {
        let service = Service::new();
        let task = service.create_task("Example Task".to_string());

        assert_eq!(task.id, 0);
        assert_eq!(task.name, "Example Task");
    }

    #[test]
    fn test_service_list_tasks() {
        let service = Service::new();
        service.create_task("Example Task".to_string());

        let tasks = service.list_tasks();

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, 0);
        assert_eq!(tasks[0].name, "Example Task");
    }

    #[test]
    fn test_service_get_task_success() {
        let service = Service::new();
        service.create_task("Example Task".to_string());

        let task = service.get_task(0).unwrap();

        assert_eq!(task.id, 0);
        assert_eq!(task.name, "Example Task");
    }

    #[test]
    fn test_service_get_task_failure() {
        let service = Service::new();
        let task = service.get_task(0);

        match task {
            Some(_) => panic!("Task should not exist"),
            None => assert!(true),
        };
    }

    #[test]
    fn test_service_delete_task_success() {
        let service = Service::new();
        service.create_task("Example Task".to_string());

        let tasks = service.list_tasks();
        assert_eq!(tasks.len(), 1);

        let task = service.delete_task(0).unwrap();

        assert_eq!(task.id, 0);
        assert_eq!(task.name, "Example Task");

        let tasks = service.list_tasks();
        assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn test_service_delete_task_failure() {
        let service = Service::new();
        let task = service.delete_task(0);

        match task {
            Some(_) => panic!("Task should not exist"),
            None => assert!(true),
        };
    }

    #[test]
    fn test_task_serialize() {
        let task = Task::new(0, "Example Task".to_string());
        let json = serde_json::to_string(&task).unwrap();

        assert_eq!(json, r#"{"id":0,"name":"Example Task"}"#);
    }

    #[test]
    fn test_task_deserialize() {
        let json = r#"{"id":0,"name":"Example Task"}"#;
        let task: Task = serde_json::from_str(json).unwrap();

        assert_eq!(task.id, 0);
        assert_eq!(task.name, "Example Task");
    }
}
