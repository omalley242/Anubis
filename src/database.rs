use petgraph::prelude::UnGraphMap;
use tantivy::{query::QueryParser, IndexReader, IndexWriter};

pub trait AnubisDatabaseInterface {}

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
}

pub trait AnubisIndexInterface {}
pub trait AnubisGraphInterface {}

pub struct TantivyIndexDB {
    writer: IndexWriter,
    reader: IndexReader,
    query_parse: QueryParser,
}
impl AnubisIndexInterface for TantivyIndexDB {}

pub struct PetgraphGraphDB<'a>(UnGraphMap<&'a str, ()>);

impl AnubisGraphInterface for PetgraphGraphDB<'_> {}
