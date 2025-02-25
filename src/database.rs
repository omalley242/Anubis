use std::path::Path;

use petgraph::prelude::UnGraphMap;
use tantivy::{query::QueryParser, IndexReader, IndexWriter};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait AnubisDatabaseInterface {
    fn new(path: Option<&Path>) -> Result<Box<Self>>;
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
    fn new(path: Option<&Path>) -> Result<Box<Self>> {}
}

pub trait AnubisIndexInterface {
    fn new(path: Option<&Path>) -> Result<Box<Self>>;
}
pub trait AnubisGraphInterface {
    fn new(path: Option<&Path>) -> Result<Box<Self>>;
}

pub struct TantivyIndexDB {
    writer: IndexWriter,
    reader: IndexReader,
    query_parse: QueryParser,
}

impl AnubisIndexInterface for TantivyIndexDB {
    fn new(path: Option<&Path>) -> Result<Box<Self>> {}
}

pub struct PetgraphGraphDB<'a>(UnGraphMap<&'a str, ()>);

impl AnubisGraphInterface for PetgraphGraphDB<'_> {
    fn new(path: Option<&Path>) -> Result<Box<Self>> {}
}
