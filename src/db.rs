use crate::common::Block;
use rusqlite::{params, Connection, Rows};
use serde_json::from_str;
use serde_rusqlite::from_rows;
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

pub fn insert_block(
    db: &Connection,
    block: &Block,
    file_path: &PathBuf,
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

pub fn insert_page(db: &Connection, block: &Block) -> Result<(), Box<dyn std::error::Error>> {
    db.execute(
        "insert into blocks (header, page_content) values (?1, ?2)",
        params![block.info.name, serde_json::to_string(block)?],
    )?;
    Ok(())
}

pub fn retrieve_blocks(db: &Connection) -> Result<Vec<Block>, Box<dyn std::error::Error>> {
    let mut statement = db.prepare("select * from blocks")?;
    let rows = statement.query([])?;
    let rows_data = from_rows::<(usize, String, PathBuf, String)>(rows)
        .collect::<Result<Vec<(usize, String, PathBuf, String)>, _>>()?;
    let deserialized = rows_data
        .iter()
        .map(|row_data| serde_json::from_str::<Block>(&row_data.3))
        .collect::<Result<Vec<Block>, _>>()?;
    return Ok(deserialized);
}
