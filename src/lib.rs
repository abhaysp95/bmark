use std::{
    fs::{self, File},
    path::{Path, PathBuf}, time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result};
use rusqlite::{params, Connection, Error::{self, QueryReturnedNoRows}, OptionalExtension, Row};
use uuid::{NoContext, Timestamp};

mod date;

pub enum BMarkTask {
    Setup {
        dbpath: Option<PathBuf>,
    },
    Add {
        url: String,
        name: String,
        tags: Vec<String>,
        desc: Option<String>,
        category: Option<PathBuf>,
    },
    List {
        output: Option<OutputType>,
        cols: ListColumn,
        tag_mode: TagMode,
    },
}

#[derive(Clone)]
pub enum OutputType {
    All,
    Tag(Vec<String>),
}

#[derive(Clone)]
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

#[allow(dead_code)]
pub struct Bookmark {
    url: String,
    name: String,
    tag: Vec<String>,
    desc: Option<String>,
    category: Option<String>,
}

fn create_table(conn: &Connection, schema: &str) -> Result<()> {
    conn.execute(schema, ())?;
    Ok(())
}

pub struct BMark {
    conn: Connection,
}

impl BMark {
    pub fn new<P>(path: P, perform_setup: bool) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        if perform_setup {
            _ = fs::create_dir_all(path.as_ref().parent().unwrap())?;
            _ = File::create(path.as_ref())?;
        }
        return Ok(BMark {
            conn: get_db_connection(Some(&path.as_ref().to_path_buf()))
                .expect("Connection to db needs to be created"),
        });
    }

    pub fn setup(&self) -> Result<()> {
        let bmark_schema = "CREATE TABLE bmark ( id TEXT PRIMARY KEY, url TEXT NOT NULL, name TEXT, description TEXT, category TEXT, added_at TEXT NOT NULL DEFAULT current_timestamp);";
        let tag_schema = "CREATE TABLE tag ( id TEXT PRIMARY KEY, name TEXT UNIQUE NOT NULL, added_at TEXT NOT NULL DEFAULT current_timestamp);";
        let bmark_tag_schema = "CREATE TABLE bmark_tag ( bmark_id TEXT, tag_id TEXT, created_at TEXT NOT NULL DEFAULT current_timestamp, FOREIGN KEY (bmark_id) REFERENCES bmark(id), FOREIGN KEY (tag_id) REFERENCES tag(id), PRIMARY KEY (bmark_id, tag_id));";

        create_table(&self.conn, bmark_schema)?;
        create_table(&self.conn, tag_schema)?;
        create_table(&self.conn, bmark_tag_schema)?;

        Ok(())
    }

    pub fn insert(
        &mut self,
        url: &str,
        name: Option<&str>,
        tags: Vec<&str>,
        desc: Option<&str>,
        category: Option<&str>,
    ) -> Result<()> {
        let mut tag_uuids: Vec<String> = vec![];

        let tags_not_present = tags.iter().filter(|&&t| {
            let query_tag = format!("Select id from tag where name='{}'", t);
            let tag_id = self.conn.query_row(&query_tag, [], |row| row.get::<_, String>(0)).optional().unwrap();
            if let Some(tag_id) = &tag_id {
                tag_uuids.push(tag_id.clone());
            }
            return tag_id.is_none();
        }).collect::<Vec<_>>();

        // insert bmark
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Duration befor Unix Epoch");
        let ts = Timestamp::from_unix(NoContext, epoch.as_secs(), 0);
        let bmark_uuid = uuid::Uuid::new_v7(ts).hyphenated().to_string();

        let tx = self.conn.transaction()?;
        tx.execute("INSERT INTO bmark (id, url, name, description, category) VALUES (?1, ?2, ?3, ?4, ?5)",
             params![bmark_uuid, url, name, desc, category])?;

        for tag in tags_not_present {
            let uuid = uuid::Uuid::new_v7(ts).hyphenated().to_string();
            tag_uuids.push(uuid.clone());
            tx.execute("INSERT INTO tag (id, name) VALUES(?1, ?2)", params![uuid, tag])?;
        }

        // make bmark-tag relation
        for tag_uuid in tag_uuids {
            tx.execute("INSERT INTO bmark_tag (bmark_id, tag_id) VALUES(?1, ?2)", params![bmark_uuid, tag_uuid])?;
        }

        Ok(tx.commit()?)
    }

    fn make_row(row: &Row, range: usize) -> Result<String, Error> {
        let mut res = String::new();
        for i in 0..=range {
            res.push_str(&row.get::<_, String>(i)?.to_string());
            if i < range {
                res.push('|');
            }
        }

        return Ok(res);
    }

    pub fn list(&self, output_type: &OutputType, column: &ListColumn) -> Result<()> {
        _ = output_type;
        _ = column;
        let mut stmt = self.conn.prepare("SELECT b.url, b.name, t.name, b.description, b.category FROM bmark b LEFT JOIN bmark_tag bt ON bt.bmark_id=b.id LEFT JOIN tag t ON bt.tag_id=t.id")?;
        let rows = stmt.query_map([], |row| Self::make_row(row, 4))?;

        for row in rows {
            if let Ok(row) = row {
                println!("{}", row);
            }
        }

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
    let res_bmark = stmt.query_row(&[tables[0]], |row| row.get::<_, String>(0));
    if res_bmark == Err(QueryReturnedNoRows) {
        return Ok(false);
    }
    let res_tag = stmt.query_row(&[tables[1]], |row| row.get::<_, String>(0));
    if res_tag == Err(QueryReturnedNoRows) {
        return Ok(false)
    }
    let res_bmark_tag = stmt.query_row(&[tables[2]], |row| row.get::<_, String>(0));
    if res_bmark_tag == Err(QueryReturnedNoRows) {
        return Ok(false); }

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
