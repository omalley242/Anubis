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

        let language_config = self
            .database
            .get_lang(header)
            .ok_or(AnubisError::ConfigError(
                "Language Config not found".to_string(),
            ))?;

        let html_string = &block
            .content
            .iter()
            .map(|content| self.render_block_content(content, language_config))
            .collect::<Result<String, Box<dyn std::error::Error>>>()?;

        let neighbors = self.database.get_connections(&block.info.name).ok_or(
            AnubisError::ConnectionsNotFound("Block not found in graph".to_string()),
        )?;

        let mut context = Context::new();
        context.insert("html", &html_string);
        context.insert("neighbors", &neighbors);
        let rendered_string = self
            .tera
            .render(&format!("{}.html", block.info.template_name), &context)?;

        Ok((header.clone(), rendered_string))
    }

    fn render_block_content(
        &self,
        content: &BlockContent,
        language_config: &LanguageConfig,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let string_content = match content {
            BlockContent::Code(data) => Self::render_code(data, language_config),
            BlockContent::Markdown(data) => Self::render_markdown(data),
            BlockContent::Link(data) => self.render_link(data),
            BlockContent::Embed(data) => self.render_embed(data)?.1,
        };
        Ok(string_content)
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
