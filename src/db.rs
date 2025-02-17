use crate::common::{AnubisError, Block};
use rusqlite::{params, Connection};
use serde_rusqlite::from_rows;
use std::path::Path;
use std::path::PathBuf;

pub fn block_db() -> Result<Connection, Box<dyn std::error::Error>> {
    let db = Connection::open("anubis.db")?;

    db.execute("drop table if exists blocks", params![])?;
    db.execute(
        "create table if not exists blocks (
        id integer primary key autoincrement,
        header text not null unique,
        file_origin text not null,
        block_content text not null
    );
    ",
        params![],
    )?;

    Ok(db)
}

pub fn pages_db() -> Result<Connection, Box<dyn std::error::Error>> {
    let db = Connection::open("anubis.db")?;

    db.execute("drop table if exists pages", params![])?;
    db.execute(
        "create table if not exists pages (
        id integer primary key autoincrement,
        header text not null unique,
        page_content text not null
    );
    ",
        params![],
    )?;

    Ok(db)
}

pub fn open_db() -> Result<Connection, Box<dyn std::error::Error>> {
    Ok(Connection::open("anubis.db")?)
}

pub fn insert_block(
    db: &Connection,
    block: &Block,
    file_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    db.execute(
        "insert into blocks (header, file_origin, block_content) values (?1, ?2, ?3)",
        params![
            block.info.name,
            file_path.to_str(),
            serde_json::to_string(block)?
        ],
    )?;
    Ok(())
}

pub fn get_block(
    db: &Connection,
    header_string: &String,
) -> Result<Block, Box<dyn std::error::Error>> {
    let mut statement = db.prepare("select * from blocks where header = (?1)")?;
    let block = statement.query([header_string])?;
    let block_data = from_rows::<(usize, String, PathBuf, String)>(block);
    let block_result = block_data.last();

    if block_result.is_some() {
        Ok(serde_json::from_str(&block_result.unwrap()?.3)?)
    } else {
        Err(AnubisError::BlockNotFoundError(
            "Block not found in db".to_string(),
        ))?
    }
}

pub fn insert_page(
    db: &Connection,
    header_string: &String,
    page_content: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    db.execute(
        "insert into pages (header, page_content) values (?1, ?2)",
        params![header_string, page_content],
    )?;
    Ok(())
}

pub fn get_page(
    db: &Connection,
    header_string: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut statement = db.prepare("select * from pages where header = (?1)")?;
    let page = statement.query([header_string])?;
    let page_data = from_rows::<(usize, String, String)>(page);
    let page_result = page_data.last();

    if page_result.is_some() {
        Ok(page_result.unwrap()?.2)
    } else {
        Err(AnubisError::PageNotFoundError(
            "Page not found from db".to_string(),
        ))?
    }
}

pub fn retrieve_rows(db: &Connection) -> Result<Vec<(PathBuf, Block)>, Box<dyn std::error::Error>> {
    let mut statement = db.prepare("select * from blocks")?;
    let rows = statement.query([])?;
    let rows_data = from_rows::<(usize, String, PathBuf, String)>(rows)
        .collect::<Result<Vec<(usize, String, PathBuf, String)>, _>>()?;
    let deserialized_rows = rows_data
        .iter()
        .map(|data| {
            (
                data.2.clone(),
                serde_json::from_str::<Block>(&data.3).unwrap(),
            )
        })
        .collect::<Vec<(PathBuf, Block)>>();

    Ok(deserialized_rows)
}
