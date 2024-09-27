use crate::{database::get_connection, model::{Task, TaskBuilder}};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::sync::Arc;
use std::sync::Mutex;

pub trait Repository {
    fn get_tasks(&self) -> Vec<Task>;
    fn add_task(&self, name: String) -> Task;
    fn get_task(&self, id: u32) -> Option<Task>;
    fn delete_task(&self, id: u32) -> Option<Task>;
}

pub struct Database {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl Database {
    pub fn new(pool: Arc<Pool<SqliteConnectionManager>>) -> Self {
        let connection = get_connection(&pool);

        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )",
                [],
            )
            .expect("Failed to create tasks table");

        Database { pool }
    }
}

impl Repository for Database {
    fn get_tasks(&self) -> Vec<Task> {
        let connection = get_connection(&self.pool);

        let mut statement = connection
            .prepare("SELECT id, name FROM tasks")
            .expect("Failed to prepare statement");

        let tasks = statement
            .query_map([], |row| {
                let id = row.get(0)?;
                let name: String = row.get(1)?;
                Ok(TaskBuilder::new().id(id).name(&name).build())
            })
            .expect("Failed to query tasks")
            .map(|result| result.expect("Failed to map task"))
            .collect();

        tasks
    }

    fn add_task(&self, name: String) -> Task {
        let connection = get_connection(&self.pool);

        connection
            .execute("INSERT INTO tasks (name) VALUES (?)", [name.clone()])
            .expect("Failed to insert task");

        let id = u32::try_from(connection.last_insert_rowid()).expect("Failed to convert id");

        TaskBuilder::new().id(id).name(&name).build()
    }

    fn get_task(&self, id: u32) -> Option<Task> {
        let connection = get_connection(&self.pool);

        let mut statement = connection
            .prepare("SELECT id, name FROM tasks WHERE id = ?")
            .expect("Failed to prepare statement");

        statement
            .query_row([id], |row| {
                let id = row.get(0)?;
                let name: String = row.get(1)?;
                Ok(TaskBuilder::new().id(id).name(&name).build())
            })
            .ok()
    }

    fn delete_task(&self, id: u32) -> Option<Task> {
        let connection = get_connection(&self.pool);

        let mut statement = connection
            .prepare("DELETE FROM tasks WHERE id = ?")
            .expect("Failed to prepare statement");

        let task = self.get_task(id);

        statement.execute([id]).expect("Failed to delete task");

        task
    }
}

pub struct Memory {
    tasks: Mutex<Vec<Task>>,
    count: Mutex<u32>,
}

impl Memory {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Memory {
            tasks: Mutex::new(Vec::new()),
            count: Mutex::new(0),
        }
    }
}

impl Repository for Memory {
    fn get_tasks(&self) -> Vec<Task> {
        let tasks = self.tasks.lock().unwrap();
        tasks.clone()
    }

    fn add_task(&self, name: String) -> Task {
        let mut tasks = self.tasks.lock().unwrap();
        let mut count = self.count.lock().unwrap();

        let id = *count;
        *count += 1;

        let task = TaskBuilder::new().id(id).name(&name).build();
        tasks.push(task.clone());

        task
    }

    fn get_task(&self, id: u32) -> Option<Task> {
        let tasks = self.tasks.lock().unwrap();
        tasks.iter().find(|task| task.id() == id).cloned()
    }

    fn delete_task(&self, id: u32) -> Option<Task> {
        let mut tasks = self.tasks.lock().unwrap();
        let task = tasks.iter().position(|task| task.id() == id);

        task.map(|index| tasks.remove(index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_new() {
        let pool = Arc::new(Pool::new(SqliteConnectionManager::file(":memory:")).unwrap());
        let database = Database::new(pool);

        let tasks = database.get_tasks();

        assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn test_database_add_task() {
        let pool = Arc::new(Pool::new(SqliteConnectionManager::file(":memory:")).unwrap());
        let database = Database::new(pool);

        let task = database.add_task("Example Task".to_string());

        assert_eq!(task.id(), 1);
        assert_eq!(task.name(), "Example Task");
    }

    #[test]
    fn test_database_get_task() {
        let pool = Arc::new(Pool::new(SqliteConnectionManager::file(":memory:")).unwrap());
        let database = Database::new(pool);

        database.add_task("Example Task".to_string());

        let task = database.get_task(1).unwrap();

        assert_eq!(task.id(), 1);
        assert_eq!(task.name(), "Example Task");
    }

    //#[test]
    //fn test_database_delete_task() {
    //    let pool = Arc::new(Pool::new(SqliteConnectionManager::file(":memory:")).unwrap());
    //    let database = Database::new(pool);
    //
    //    database.add_task("Example Task".to_string());
    //
    //    let task = database.delete_task(1).unwrap();
    //
    //    assert_eq!(task.id, 1);
    //    assert_eq!(task.name, "Example Task");
    //
    //    let tasks = database.get_tasks();
    //
    //    assert_eq!(tasks.len(), 0);
    //}
}
