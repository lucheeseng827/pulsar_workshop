extern crate rusqlite;

use rusqlite::{Connection, Result, NO_PARAMS};

// Open a connection to a SQLite database
let conn = Connection::open("my_database.db")?;

// Enable the write-ahead log mode for the connection
conn.pragma_update(None, "journal_mode", "WAL")?;

// Execute some SQL statements that modify the database
conn.execute("INSERT INTO users (name) VALUES (?)", &["Alice"])?;
conn.execute("UPDATE users SET email = ? WHERE name = ?", &["alice@example.com", "Alice"])?;

// Commit the transaction to make the changes permanent
conn.execute("COMMIT", NO_PARAMS)?;

// Close the connection to the database
conn.close()?;
