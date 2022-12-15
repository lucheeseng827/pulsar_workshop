extern crate rusqlite;

use std::fs;
use std::path::Path;

use rusqlite::{Connection, Result, NO_PARAMS};

struct Queue {
    // The path to the SQLite database file that stores the queue data
    db_path: String,

    // The connection to the SQLite database
    conn: Connection,
}

impl Queue {
    // Open a connection to the SQLite database and enable the WAL mode
    fn open(db_path: &str) -> Result<Queue> {
        // Create the directory for the database file if it doesn't already exist
        let db_dir = Path::new(&db_path).parent().unwrap();
        if !db_dir.exists() {
            fs::create_dir_all(db_dir)?;
        }

        // Open a connection to the SQLite database
        let conn = Connection::open(db_path)?;

        // Enable the write-ahead log mode for the connection
        conn.pragma_update(None, "journal_mode", "WAL")?;

        // Initialize the "queue" table if it doesn't already exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS queue (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                value TEXT NOT NULL
            )",
            NO_PARAMS,
        )?;

        Ok(Queue {
            db_path: db_path.to_string(),
            conn,
        })
    }

    // Push a new value onto the queue
    fn push(&self, value: &str) -> Result<()> {
        self.conn.execute("INSERT INTO queue (value) VALUES (?)", &[value])?;
        Ok(())
    }

    // Pop the next value from the queue
    fn pop(&self) -> Result<Option<String>> {
        // Begin a transaction to pop the next value from the queue
        self.conn.execute("BEGIN", NO_PARAMS)?;

        // Select the next value from the queue
        let mut stmt = self
            .conn
            .prepare("SELECT value FROM queue WHERE id = (SELECT min(id) FROM queue)")?;
        let rows = stmt.query_map(NO_PARAMS, |row| row.get(0))?;

        // Delete the value from the queue
        let mut deleted = false;
        for row in rows {
            let value: String = row?;
            self.conn.execute("DELETE FROM queue WHERE value = ?", &[&value])?;
            deleted = true;
            break;
        }

        // If a value was selected, commit the transaction and return the value
        if deleted {
            self.conn.execute("COMMIT", NO_PARAMS)?;
            Ok(Some(value))
        } else {
            // If no value was selected, roll back the transaction and return None
            self.conn.execute("ROLLBACK", NO_PARAMS)?;
            Ok(None)
        }
    }

    // Close the connection to the SQLite database
    fn close(
