use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait AnubisParserInterface {
    fn new(path: Option<&PathBuf>) -> Self;
}

pub struct AnubisParser {}

impl AnubisParserInterface for AnubisParser {
    fn new(path: Option<&PathBuf>) -> Self {
        AnubisParser {}
    }
}
