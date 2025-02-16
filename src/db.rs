use crate::common::Block;
use rusqlite::{params, Connection};
use std::path::PathBuf;

pub fn block_db() -> Result<Connection, Box<dyn std::error::Error>> {
    let db = Connection::open("blocks.db")?;

    db.execute("drop table blocks", params![])?;
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
