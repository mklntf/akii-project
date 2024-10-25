use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Serialize, Deserialize, ToSchema)]
pub struct Task {
    id: u32,
    name: String,
}

impl Task {
    pub fn id(&self) -> u32 {
        self.id
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct TaskBuilder {
    id: u32,
    name: String,
}

impl TaskBuilder {
    pub fn new() -> Self {
        TaskBuilder {
            id: 0,
            name: String::new(),
        }
    }

    pub fn id(&mut self, id: u32) -> &mut Self {
        self.id = id;
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn build(&self) -> Task {
        Task {
            id: self.id,
            name: self.name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_new() {
        let task = TaskBuilder::new().id(0).name("Example Task").build();

        assert_eq!(task.id, 0);
        assert_eq!(task.name, "Example Task");
    }

    #[test]
    fn test_task_serialize() {
        let task = TaskBuilder::new().id(0).name("Example Task").build();
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
