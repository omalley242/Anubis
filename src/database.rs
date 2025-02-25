use std::{path::PathBuf, str::FromStr};

use petgraph::prelude::UnGraphMap;
use tantivy::{
    query::QueryParser,
    schema::{Schema, STORED, TEXT},
    Index, IndexReader, IndexWriter,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait AnubisDatabaseInterface {
    fn new(path: Option<&PathBuf>) -> Result<Box<Self>>;
}

pub struct AnubisDatabase<A, B>
where
    A: AnubisGraphInterface,
    B: AnubisIndexInterface,
{
    graphdb: A,
    indexdb: B,
}

impl<A: AnubisGraphInterface, B: AnubisIndexInterface> AnubisDatabaseInterface
    for AnubisDatabase<A, B>
{
    fn new(path: Option<&PathBuf>) -> Result<Box<Self>> {
        let graphdb = *A::new(path)?;
        let indexdb = *B::new(path)?;
        Ok(AnubisDatabase { indexdb, graphdb }.into())
    }
}

pub trait AnubisIndexInterface {
    fn new(path: Option<&PathBuf>) -> Result<Box<Self>>;
}
pub trait AnubisGraphInterface {
    fn new(path: Option<&PathBuf>) -> Result<Box<Self>>;
}

pub struct TantivyIndexDB {
    writer: IndexWriter,
    reader: IndexReader,
    query_parse: QueryParser,
}

impl AnubisIndexInterface for TantivyIndexDB {
    fn new(path: Option<&PathBuf>) -> Result<Box<Self>> {
        //create schema
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("block", TEXT | STORED);
        let schema = schema_builder.build();
        //create index or open index
        let index: Index;

        if let Some(path) = path {
            index = Index::open_in_dir(path)?;
        } else {
            index = Index::create_in_dir(PathBuf::from_str("./")?, schema)?;
        }

        Ok(TantivyIndexDB {
            writer: index.writer(50_000_000)?,
            reader: index.reader()?,
            query_parse: QueryParser::for_index(&index, vec![]),
        }
        .into())
    }
}

pub struct PetgraphGraphDB<'a>(UnGraphMap<&'a str, ()>);

impl AnubisGraphInterface for PetgraphGraphDB<'_> {
    fn new(path: Option<&PathBuf>) -> Result<Box<Self>> {
        Ok(PetgraphGraphDB(UnGraphMap::new()).into())
    }
}
