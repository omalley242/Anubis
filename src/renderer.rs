use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait AnubisRendererInterface {
    fn new(path: Option<&PathBuf>) -> Self;
}

pub struct AnubisRenderer {}

impl AnubisRendererInterface for AnubisRenderer {
    fn new(path: Option<&PathBuf>) -> Self {
        AnubisRenderer {}
    }
}
