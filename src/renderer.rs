use std::collections::HashSet;

use crate::{
    common::{Anubis, AnubisError, Block, BlockContent},
    config::LanguageConfig,
    db::HtmlDB,
};
use comrak::{markdown_to_html, ExtensionOptions, Options};
use tera::Context;

pub trait AnubisRenderer {
    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn render_block(&self, block: &Block) -> Result<(String, String), Box<dyn std::error::Error>>;
    fn render_block_content(
        &self,
        content: &BlockContent,
        lang_config: &LanguageConfig,
    ) -> Result<String, Box<dyn std::error::Error>>;
    fn render_code(code_string: &str, lang_config: &LanguageConfig) -> String;
    fn render_link(&self, link_string: &str) -> String;
    fn render_markdown(markdown_string: &str) -> String;
    fn render_embed(
        &self,
        embed_string: &str,
    ) -> Result<(String, String), Box<dyn std::error::Error>>;
    fn render_block_contents(
        &self,
        block: &Block,
        language_config: &LanguageConfig,
    ) -> Result<String, Box<dyn std::error::Error>>;
    fn apply_template(
        &self,
        html: &str,
        neighbors: &HashSet<String>,
        template_name: &str,
    ) -> Result<String, tera::Error>;
    fn get_language_config(&self, header: &str) -> Result<&LanguageConfig, AnubisError>;
    fn get_neighbors(&self, header: &str) -> Result<&HashSet<String>, AnubisError>;
}

impl AnubisRenderer for Anubis {
    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let blocks = self.database.block_db.values();
        let html_db = blocks
            .map(|block| self.render_block(block))
            .collect::<Result<HtmlDB, Box<dyn std::error::Error>>>();
        self.database.html_db = html_db?;
        self.database.save("./anubis.db")?;
        Ok(())
    }

    fn render_block(&self, block: &Block) -> Result<(String, String), Box<dyn std::error::Error>> {
        let header = &block.info.name;
        let language_config = self.get_language_config(header)?;
        let html_string = self.render_block_contents(block, language_config)?;
        let neighbors = self.get_neighbors(header)?;
        let rendered_string =
            self.apply_template(&html_string, neighbors, &block.info.template_name)?;
        Ok((header.clone(), rendered_string))
    }

    fn get_language_config(&self, header: &str) -> Result<&LanguageConfig, AnubisError> {
        self.database
            .get_lang(header)
            .ok_or(AnubisError::ConnectionsNotFound(
                "Config Not Found in DB".to_string(),
            ))
    }

    fn get_neighbors(&self, header: &str) -> Result<&HashSet<String>, AnubisError> {
        self.database
            .get_connections(header)
            .ok_or(AnubisError::ConnectionsNotFound(
                "Block not found in graph".to_string(),
            ))
    }

    fn apply_template(
        &self,
        html: &str,
        neighbors: &HashSet<String>,
        template_name: &str,
    ) -> Result<String, tera::Error> {
        let mut context = Context::new();
        context.insert("html", html);
        context.insert("neighbors", neighbors);
        self.tera
            .render(&format!("{}.html", template_name), &context)
    }

    fn render_block_contents(
        &self,
        block: &Block,
        language_config: &LanguageConfig,
    ) -> Result<String, Box<dyn std::error::Error>> {
        block
            .content
            .iter()
            .map(|content| self.render_block_content(content, language_config))
            .collect::<Result<String, Box<dyn std::error::Error>>>()
    }

    fn render_block_content(
        &self,
        content: &BlockContent,
        language_config: &LanguageConfig,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(match content {
            BlockContent::Code(data) => Self::render_code(data, language_config),
            BlockContent::Markdown(data) => Self::render_markdown(data),
            BlockContent::Link(data) => self.render_link(data),
            BlockContent::Embed(data) => self.render_embed(data)?.1,
        })
    }

    fn render_code(code_string: &str, lang_config: &LanguageConfig) -> String {
        let language = lang_config.language.clone();
        markdown_to_html(
            &format!("```{language}{code_string}```"),
            &Options::default(),
        )
    }

    fn render_link(&self, link_string: &str) -> String {
        let url = &self.config.url;
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
        &self,
        embed_string: &str,
    ) -> Result<(String, String), Box<dyn std::error::Error>> {
        if let Some(block) = self.database.get_block(embed_string) {
            self.render_block(&block.clone())
        } else {
            Err(AnubisError::BlockNotFoundError(
                "Could not find block in database".to_string(),
            ))?
        }
    }
}
