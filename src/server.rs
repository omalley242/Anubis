use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait AnubisServerInterface {
    fn new(path: Option<&Path>) -> Self;
}

pub struct AnubisServer {}

impl AnubisServerInterface for AnubisServer {
    fn new(path: Option<&Path>) -> Self {}
}
