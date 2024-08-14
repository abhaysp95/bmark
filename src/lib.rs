use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use rusqlite::Connection;

mod date;

pub enum BMarkTask {
    Setup {
        dbpath: Option<PathBuf>,
    },
    Add {
        url: String,
        tags: Vec<String>,
        desc: Option<String>,
        category: Option<PathBuf>,
        // date: String,
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

fn create_table(conn: &Connection, schema: &str) -> Result<()> {
    conn.execute(schema, ())?;
    Ok(())
}

pub struct BMark {
    conn: Connection,
}

impl BMark {
    pub fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        _ = fs::create_dir_all(path.as_ref().parent().unwrap())?;
        _ = File::create(path.as_ref())?;
        return Ok(BMark {
            conn: get_db_connection(Some(&path.as_ref().to_path_buf()))
                .expect("Connection to db needs to be created"),
        });
    }

    pub fn setup(&self) -> Result<()> {
        let bmark_schema = "CREATE TABLE bmark ( id TEXT PRIMARY KEY, url TEXT NOT NULL, description TEXT, category TEXT, name TEXT, added_at DATETIME);";
        let tag_schema = "CREATE TABLE tag ( id TEXT PRIMARY KEY, name TEXT UNIQUE NOT NULL);";
        let bmark_tag_schema = "CREATE TABLE bmark_tag ( bmark_id TEXT, tag_id TEXT, FOREIGN KEY (bmark_id) REFERENCES bmark(id), FOREIGN KEY (tag_id) REFERENCES tag(id), PRIMARY KEY (bmark_id, tag_id));";

        create_table(&self.conn, bmark_schema)?;
        create_table(&self.conn, tag_schema)?;
        create_table(&self.conn, bmark_tag_schema)?;

        Ok(())
    }

    pub fn insert(
        &self,
        url: &str,
        tags: Vec<&str>,
        desc: Option<&str>,
        category: Option<&str>,
    ) -> Result<()> {
        let datetime = date::get_current_datetime();
        println!("url: {}", url);
        println!("tags: {}", tags.join(", "));
        println!("desc: {}", desc.unwrap_or("Nothing"));
        println!("category: {}", category.unwrap_or("Nothing"));
        println!("datetime: {}", datetime);
        Ok(())
    }
}

// Perform db operation
fn get_db_connection(path: Option<&PathBuf>) -> Result<Connection> {
    match path {
        Some(p) => Connection::open(p).with_context(|| format!("Couldn't open connection to db")),
        None => Connection::open_in_memory()
            .with_context(|| format!("Couldn't open connection to db in memory")),
    }
}

pub fn is_setup_done() -> Result<bool> {
    let path = Path::new("./local/bmark/bmark.db");
    if !path.exists() {
        return Ok(false);
    }
    let tables = vec!["bmark", "tag", "bmark_tag"];
    let conn = get_db_connection(Some(&path.to_path_buf())).unwrap();
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE name=?1")?;
    let res_bmark = stmt.query_row(&[tables[0]], |row| row.get::<_, String>(0))?;
    if &res_bmark != tables[0] {
        return Ok(false);
    }
    let res_tag = stmt.query_row(&[tables[1]], |row| row.get::<_, String>(0))?;
    if &res_tag != tables[1] {
        return Ok(false);
    }
    let res_bmark_tag = stmt.query_row(&[tables[2]], |row| row.get::<_, String>(0))?;
    if &res_bmark_tag != tables[2] {
        return Ok(false);
    }

    return Ok(true);
}

#[test]
fn is_table_created() -> Result<()> {
    let conn = get_db_connection(None)?;
    let table_name = "my_table";
    let schema = &format!("CREATE TABLE {} ( id TEXT PRIMARY KEY, url TEXT NOT NULL, description TEXT, category TEXT, name TEXT);", table_name);
    create_table(&conn, schema)?;

    // query whether table exists
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE name=?1")?;
    let res = stmt.query_row(&[table_name], |row| row.get::<_, String>(0))?;

    assert_eq!(table_name, &res);

    Ok(())
}
