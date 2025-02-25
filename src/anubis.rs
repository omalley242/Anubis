use crate::{
    config::AnubisConfigInterface, database::AnubisDatabaseInterface,
    parser::AnubisParserInterface, renderer::AnubisRendererInterface,
    server::AnubisServerInterface,
};

trait AnubisInterface {
    fn parse();
    fn render();
    async fn serve();
}

struct Anubis<A, B, C, D, E>
where
    A: AnubisConfigInterface,
    B: AnubisDatabaseInterface,
    C: AnubisParserInterface,
    D: AnubisRendererInterface,
    E: AnubisServerInterface,
{
    config: A,
    database: B,
    parser: C,
    renderer: D,
    server: E,
}

impl<A, B, C, D, E> AnubisInterface for Anubis<A, B, C, D, E>
where
    A: AnubisConfigInterface,
    B: AnubisDatabaseInterface,
    C: AnubisParserInterface,
    D: AnubisRendererInterface,
    E: AnubisServerInterface,
{
    fn parse() {}

    fn render() {}

    async fn serve() {}
}
