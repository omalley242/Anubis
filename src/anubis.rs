use crate::database::AnubisDatabase;

struct Anubis<A, B, C, D, E, F>
where
    A: AnubisConfigInterface,
{
    config: A,
    database: AnubisDatabase<B, C>,
    parser: D,
    renderer: E,
    server: F,
}

impl Anubis {
    fn parse() {}

    fn render() {}

    async fn serve() {}
}
