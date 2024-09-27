use crate::model::Task;
use crate::repository::Repository;

pub struct Service {
    repository: Box<dyn Repository + Send + Sync>,
}

impl Service {
    pub fn new(repository: Box<dyn Repository + Send + Sync>) -> Self {
        Service { repository }
    }

    pub fn list_tasks(&self) -> Vec<Task> {
        let tasks = self.repository.get_tasks();
        tasks.clone()
    }

    pub fn create_task(&self, name: String) -> Task {
        self.repository.add_task(name)
    }

    pub fn get_task(&self, id: u32) -> Option<Task> {
        self.repository.get_task(id)
    }

    pub fn delete_task(&self, id: u32) -> Option<Task> {
        self.repository.delete_task(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository;

    #[test]
    fn test_service_new() {
        let repository = repository::Memory::new();
        let service = Service::new(Box::new(repository));

        let tasks = service.list_tasks();
        assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn test_service_create_task() {
        let repository = repository::Memory::new();
        let service = Service::new(Box::new(repository));
        let task = service.create_task("Example Task".to_string());

        assert_eq!(task.id(), 0);
        assert_eq!(task.name(), "Example Task");
    }

    #[test]
    fn test_service_list_tasks() {
        let repository = repository::Memory::new();
        let service = Service::new(Box::new(repository));
        service.create_task("Example Task".to_string());

        let tasks = service.list_tasks();

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id(), 0);
        assert_eq!(tasks[0].name(), "Example Task");
    }

    #[test]
    fn test_service_get_task_success() {
        let repository = repository::Memory::new();
        let service = Service::new(Box::new(repository));
        service.create_task("Example Task".to_string());

        let task = service.get_task(0).unwrap();

        assert_eq!(task.id(), 0);
        assert_eq!(task.name(), "Example Task");
    }

    #[test]
    fn test_service_get_task_failure() {
        let repository = repository::Memory::new();
        let service = Service::new(Box::new(repository));

        assert!(service.get_task(0).is_none());
    }

    #[test]
    fn test_service_delete_task_success() {
        let repository = repository::Memory::new();
        let service = Service::new(Box::new(repository));
        service.create_task("Example Task".to_string());

        let tasks = service.list_tasks();
        assert_eq!(tasks.len(), 1);

        let task = service.delete_task(0).unwrap();

        assert_eq!(task.id(), 0);
        assert_eq!(task.name(), "Example Task");

        let tasks = service.list_tasks();
        assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn test_service_delete_task_failure() {
        let repository = repository::Memory::new();
        let service = Service::new(Box::new(repository));

        assert!(service.delete_task(0).is_none());
    }
}
