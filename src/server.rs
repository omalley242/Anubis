use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait AnubisServerInterface {
    fn new() -> Self;
}

pub struct AnubisServer {}

impl AnubisServerInterface for AnubisServer {
    fn new() -> Self {
        AnubisServer {}
    }
}
