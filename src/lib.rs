use std::path::PathBuf;

use anyhow::{Context, Result};
use rusqlite::Connection;

pub enum BMarkTask {
    Add {
        url: String,
        tags: Vec<String>,
        desc: Option<String>,
        category: Option<PathBuf>,
    },
    List {
        output: Option<OutputType>,
        cols: ListColumn,
        tagMode: TagMode,
    },
}

pub enum OutputType {
    All(bool),
    Tag(Vec<String>),
}

pub enum ListColumn {
    All,
    Url,
    Tag,
    Desc,
}

pub enum TagMode {
    All,
    Any,
}

// Perform db operation
pub fn get_db_connection(path: Option<PathBuf>) -> Result<Connection> {
    match path {
        Some(p) => Connection::open(p).with_context(|| format!("Couldn't open connection to db")),
        None => Connection::open_in_memory()
            .with_context(|| format!("Couldn't open connection to db in memory")),
    }
}

pub fn create_table(conn: &Connection, name: &str) -> Result<()> {
    conn.execute(
        "CREATE TABLE ?1 (
            id TEXT PRIMARY KEY,
            url TEXT NOT NULL,
            description TEXT,
            category TEXT,
            name TEXT
            );",
        (&name,)
    )?;

    Ok(())
}
