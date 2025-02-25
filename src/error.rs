use derive_more::{Display, Error};

#[derive(Default, Debug, Display, Error)]
pub enum AnubisError {
    #[default]
    GenericError,
    ConfigError,
}
