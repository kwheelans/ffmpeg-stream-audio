use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// Pass-thru `toml::de::Error`.
    #[error("Serde Toml Error: {0}")]
    SerdeToml(#[from] toml::de::Error),

    /// Pass-thru [`std::io::Error`].
    #[error("std::io Error: {0}")]
    IO(#[from] std::io::Error),

}
