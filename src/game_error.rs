use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Reqwest error")]
    Reqwest(#[from] reqwest::Error),
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Serde JSON error")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Configuration Error")]
    ConfigError,
}
