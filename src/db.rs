use crate::{common::Block, config::LanguageConfig};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};
use tera::Context;

pub type BlockDB = HashMap<String, Block>;
pub type HtmlDB = HashMap<String, String>;
pub type GraphDB = HashMap<String, HashSet<String>>;
pub type LangDB = HashMap<String, LanguageConfig>;

// Global AnubisDatabase Should only be initalised once
#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AnubisDatabase {
    pub block_db: BlockDB,
    pub html_db: HtmlDB,
    pub graph_db: GraphDB,
    pub lang_db: LangDB,
}

impl AnubisDatabase {
    pub fn get_block(&self, header: &str) -> Option<&Block> {
        self.block_db.get(header)
    }

    pub fn get_html(&self, header: &str) -> Option<&String> {
        self.html_db.get(header)
    }

    pub fn get_connections(&self, header: &str) -> Option<&HashSet<String>> {
        self.graph_db.get(header)
    }

    pub fn get_lang(&self, header: &str) -> Option<&LanguageConfig> {
        self.lang_db.get(header)
    }

    pub fn get_context(&self, header: &str) -> Option<Context> {
        let mut context = Context::new();
        context.insert("html", self.get_html(header)?);
        context.insert("neighbors", self.get_connections(header)?);
        Some(context)
    }

    pub fn insert_block(&mut self, block: &Block, lang: &LanguageConfig) {
        let connections = block.content.iter().filter_map(|content| match content {
            crate::common::BlockContent::Embed(header) => Some(header),
            crate::common::BlockContent::Link(header) => Some(header),
            _ => None,
        });

        connections.for_each(|connection| {
            self.add_edge_undirected(block.info.name.clone(), connection.to_string());
        });

        self.lang_db.insert(block.info.name.clone(), lang.clone());
        self.block_db.insert(block.info.name.clone(), block.clone());
    }

    pub fn insert_blocks(&mut self, blocks: Vec<Block>, lang: &LanguageConfig) {
        blocks
            .iter()
            .for_each(|block| self.insert_block(block, lang));
    }

    pub fn add_edge_undirected(&mut self, node_1: String, node_2: String) {
        self.add_edge(node_1.clone(), node_2.clone());
        self.add_edge(node_2, node_1);
    }

    pub fn add_edge(&mut self, node_1: String, node_2: String) {
        if let Some(adj_1) = self.graph_db.get_mut(&node_1) {
            adj_1.insert(node_2.clone());
        } else {
            let mut new_matrix = HashSet::new();
            new_matrix.insert(node_2.clone());
            self.graph_db.insert(node_1.clone(), new_matrix);
        }
    }

    pub fn insert_html(&mut self, header: String, html_string: String) {
        self.html_db.insert(header, html_string);
    }

    pub fn save(&self, db_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(db_path)?;
        let writer = BufWriter::new(file);
        Ok(serde_json::to_writer(writer, self)?)
    }

    pub fn new(db_path: Option<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(path) = db_path {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            return Ok(serde_json::from_reader(reader)?);
        }
        Ok(AnubisDatabase::default())
    }
}
