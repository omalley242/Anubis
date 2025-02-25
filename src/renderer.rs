use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait AnubisRendererInterface {
    fn new(path: Option<&Path>) -> Self;
}

pub struct AnubisRenderer {}

impl AnubisRendererInterface for AnubisRenderer {
    fn new(path: Option<&Path>) -> Self {}
}
