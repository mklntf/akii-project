use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use std::path::Path;
use std::sync::Arc;

pub fn create_pool(file: &Path) -> Arc<Pool<SqliteConnectionManager>> {
    let manager = SqliteConnectionManager::file(file);
    let pool = Pool::new(manager).expect("Failed to create pool.");
    Arc::new(pool)
}

pub fn get_connection(
    pool: &Arc<Pool<SqliteConnectionManager>>,
) -> PooledConnection<SqliteConnectionManager> {
    pool.get().expect("Failed to get a database connection.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_connection_success() {
        let pool = create_pool(Path::new(":memory:"));

        let conn = get_connection(&pool);

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY
            )",
            [],
        )
        .unwrap();
    }
}
