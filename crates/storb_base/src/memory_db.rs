use std::sync::Arc;

use anyhow::Result;
use rusqlite::{Connection, DatabaseName};
use tokio::sync::Mutex;
use tracing::info;

pub struct MemoryDb {
    pub conn: Arc<Mutex<Connection>>,
    pub db_path: String,
}

impl MemoryDb {
    pub async fn new(db_path: &str) -> Result<Self> {
        // Create in-memory connection
        let conn_arc = Arc::new(Mutex::new(Connection::open_in_memory()?));

        let mut conn = conn_arc.lock().await;
        conn.restore(DatabaseName::Main, db_path, Some(|_p| ()))?;

        Ok(Self {
            conn: conn_arc.clone(),
            db_path: db_path.to_string(),
        })
    }

    pub async fn run_backup(&self) {
        let db_path = self.db_path.clone();
        let conn = self.conn.clone();

        info!("Backing up database");
        {
            let conn = conn.lock().await;
            conn.backup(DatabaseName::Main, &db_path, None).unwrap();
        }
        info!("Database backup complete");
    }
}

#[cfg(test)]
mod tests {
    // use std::sync::Once;
    use tempfile::NamedTempFile;

    use super::*;
    // use tokio::time::sleep;

    // This runs before any tests
    // static INIT: Once = Once::new();

    // fn setup_logging() {
    //     INIT.call_once(|| {
    //         tracing_subscriber::fmt()
    //             .with_max_level(tracing::Level::DEBUG)
    //             .with_test_writer() // This ensures output goes to the test console
    //             .init();
    //     });
    // }

    // helper function to initialize the disk database
    fn init_disk_db(db_path: &str) -> Result<()> {
        let conn = Connection::open(db_path)?;
        conn.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, value TEXT)", [])?;
        conn.execute("INSERT INTO test (value) VALUES (?1)", ["test_value"])?;
        Ok(())
    }

    #[tokio::test]
    async fn test_memory_db_creation() -> Result<()> {
        // Create a temporary file for testing
        let temp_file = NamedTempFile::new()?;
        let db_path = temp_file.path().to_str().unwrap();

        // Create an initial SQLite database with some test data
        init_disk_db(db_path)?;

        // Create MemoryDb instance
        let memory_db = MemoryDb::new(db_path).await?;

        // Verify data was loaded correctly
        let conn = memory_db.conn.lock().await;
        let mut stmt = conn.prepare("SELECT value FROM test WHERE id = 1")?;
        let value: String = stmt.query_row([], |row| row.get(0))?;

        assert_eq!(value, "test_value");
        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_db_path() {
        let result = MemoryDb::new("/nonexistent/path/db.sqlite").await;
        assert!(result.is_err());
    }
}
