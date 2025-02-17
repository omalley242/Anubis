use crate::{
    common::{find_language_config, Block, BlockContent, Config},
    db::{get_block, get_page, insert_page, pages_db, retrieve_rows},
};
use comrak::{markdown_to_html, ExtensionOptions, Options};
use rusqlite::Connection;
use std::path::PathBuf;
use tera::Tera;

pub fn render_files(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let db_con = pages_db()?;
    let rows = retrieve_rows(&db_con)?;
    let tera = Tera::new("default_templates/**/*.html")?;
    rows.iter().try_for_each(|row| {
        render_page(&db_con, config, &row.0, &row.1, &tera)?;
        Ok::<(), Box<dyn std::error::Error>>(())
    })?;
    Ok(())
}

fn render_page(
    db: &Connection,
    config: &Config,
    file_origin: &PathBuf,
    block: &Block,
    tera: &Tera,
) -> Result<String, Box<dyn std::error::Error>> {
    let page = get_page(db, &block.info.name);
    if page.is_ok() {
        return page;
    }

    let mut html_string = String::new();
    for content in &block.content {
        html_string += &match content {
            BlockContent::Code(data) => render_code(data, file_origin, config)?,
            BlockContent::Link(data) => render_link(data, &config),
            BlockContent::Markdown(data) => render_markdown(data),
            BlockContent::Embed(data) => render_embed(data, &db, &config, &file_origin, tera)?,
        };
    }

    insert_page(&db, &block.info.name, &html_string)?;
    Ok(html_string)
}

fn render_code(
    code_string: &String,
    file_origin: &PathBuf,
    config: &Config,
) -> Result<String, Box<dyn std::error::Error>> {
    let language = &find_language_config(file_origin, config)?.language;
    return Ok(markdown_to_html(
        &format!("```{language}{code_string}```"),
        &Options::default(),
    ));
}

fn render_link(link_string: &String, config: &Config) -> String {
    let url = &config.url;
    return markdown_to_html(
        &format!("[{link_string}]({url}{link_string})"),
        &Options::default(),
    );
}

fn render_markdown(markdown_string: &String) -> String {
    return markdown_to_html(
        &markdown_string,
        &Options {
            extension: ExtensionOptions {
                header_ids: Some("header".to_string()),
                math_dollars: true,
                ..Default::default()
            },
            ..Default::default()
        },
    );
}

fn render_embed(
    embed_string: &String,
    db: &Connection,
    config: &Config,
    file_origin: &PathBuf,
    tera: &Tera,
) -> Result<String, Box<dyn std::error::Error>> {
    let block = get_block(db, embed_string)?;
    return render_page(db, config, file_origin, &block, tera);
}
