use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait AnubisParserInterface {
    fn new(path: Option<&Path>) -> Self;
}

pub struct AnubisParser {}

impl AnubisParserInterface for AnubisParser {
    fn new(path: Option<&Path>) -> Self {}
}
