use std::path::PathBuf;

use anyhow::{Context, Result};
use rusqlite::Connection;

pub enum BMarkTask {
    Setup {
        dbpath: Option<PathBuf>
    },
    Add {
        url: String,
        tags: Vec<String>,
        desc: Option<String>,
        category: Option<PathBuf>,
        date: String,
    },
    List {
        output: Option<OutputType>,
        cols: ListColumn,
        tag_mode: TagMode,
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
        &format!("CREATE TABLE {} (
            id TEXT PRIMARY KEY,
            url TEXT NOT NULL,
            description TEXT,
            category TEXT,
            name TEXT
            );", name),
        ()
    )?;

    Ok(())
}

#[test]
fn is_table_created() -> Result<()> {
    let conn = get_db_connection(None)?;
    let table_name = "my_table";
    create_table(&conn, table_name)?;

    // query whether table exists
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE name=?1")?;
    let res = stmt.query_row(&[table_name], |row| row.get::<_, String>(0))?;

    assert_eq!(table_name, &res);

    Ok(())
}
