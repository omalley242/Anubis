use crate::{
    config::AnubisConfigInterface, database::AnubisDatabaseInterface,
    parser::AnubisParserInterface, renderer::AnubisRendererInterface,
    server::AnubisServerInterface,
};

pub trait AnubisInterface {
    fn parse(&self);
    fn render(&self);
    async fn serve(&self);
}

pub struct Anubis<A, B, C, D, E>
where
    A: AnubisConfigInterface,
    B: AnubisDatabaseInterface,
    C: AnubisParserInterface,
    D: AnubisRendererInterface,
    E: AnubisServerInterface,
{
    pub config: A,
    pub database: B,
    pub parser: C,
    pub renderer: D,
    pub server: E,
}

impl<A, B, C, D, E> AnubisInterface for Anubis<A, B, C, D, E>
where
    A: AnubisConfigInterface,
    B: AnubisDatabaseInterface,
    C: AnubisParserInterface,
    D: AnubisRendererInterface,
    E: AnubisServerInterface,
{
    fn parse(&self) {}

    fn render(&self) {}

    async fn serve(&self) {}
}
