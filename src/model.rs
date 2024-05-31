use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Serialize, Deserialize, ToSchema)]
pub struct Task {
    pub id: u32,
    pub name: String,
}

impl Task {
    pub fn new(id: u32, name: String) -> Self {
        Task { id, name }
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
