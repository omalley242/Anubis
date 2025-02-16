use crate::{
    common::Config,
    db::{pages_db, retrieve_blocks},
};

pub fn render_files(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let db_con = pages_db()?;
    let blocks = retrieve_blocks(&db_con);
    println!("{:?}", blocks);
    Ok(())
}
