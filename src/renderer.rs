use std::collections::HashMap;

use crate::{
    common::{Anubis, AnubisError, Block, BlockContent, Config, LanguageConfig},
    db::AnubisDatabase,
};
use comrak::{markdown_to_html, ExtensionOptions, Options};
use tera::{Context, Tera};

pub trait AnubisRenderer {
    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

impl AnubisRenderer for Anubis {
    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.database.html_db = self
            .database
            .block_db
            .iter()
            .map(|(header, block)| {
                Ok((
                    header.clone(),
                    render_block(&self.database, &self.config, &self.tera, block)?,
                ))
            })
            .collect::<Result<HashMap<String, String>, Box<dyn std::error::Error>>>()?;
        Ok(())
    }
}

fn render_block(
    database: &AnubisDatabase,
    config: &Config,
    tera: &Tera,
    block: &Block,
) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(html) = database.get_html(&block.info.name) {
        return Ok(html.clone());
    }

    let language_config = database
        .get_lang(&block.info.name)
        .ok_or(AnubisError::ConfigError(
            "Language Config not found".to_string(),
        ))?;

    let mut html_string = String::new();

    for content in &block.content {
        html_string += &match content {
            BlockContent::Code(data) => render_code(data, language_config)?,
            BlockContent::Link(data) => render_link(data, config),
            BlockContent::Markdown(data) => render_markdown(data),
            BlockContent::Embed(data) => render_embed(data, database, config, tera)?,
        };
    }

    let mut context = Context::new();
    context.insert("html", &html_string);

    let neighbors =
        database
            .get_connections(&block.info.name)
            .ok_or(AnubisError::ConnectionsNotFound(
                "Block not found in graph".to_string(),
            ))?;

    context.insert("neighbors", &neighbors);
    context.insert("html", &html_string);

    let rendered_string = tera.render(&format!("{}.html", block.info.template_name), &context)?;
    Ok(rendered_string)
}

fn render_code(
    code_string: &String,
    lang_config: &LanguageConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let language = lang_config.language.clone();
    Ok(markdown_to_html(
        &format!("```{language}{code_string}```"),
        &Options::default(),
    ))
}

fn render_link(link_string: &String, config: &Config) -> String {
    let url = &config.url;
    markdown_to_html(
        &format!("[{link_string}]({url}{link_string})"),
        &Options::default(),
    )
}

fn render_markdown(markdown_string: &str) -> String {
    markdown_to_html(
        markdown_string,
        &Options {
            extension: ExtensionOptions {
                header_ids: Some("header".to_string()),
                math_dollars: true,
                ..Default::default()
            },
            ..Default::default()
        },
    )
}

fn render_embed(
    embed_string: &str,
    database: &AnubisDatabase,
    config: &Config,
    tera: &Tera,
) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(block) = database.get_block(embed_string) {
        render_block(database, config, tera, block)
    } else {
        Err(AnubisError::BlockNotFoundError(
            "Could not find block in database".to_string(),
        ))?
    }
}
