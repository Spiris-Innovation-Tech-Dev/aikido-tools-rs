pub type Result<T> = std::result::Result<T, AikidoError>;

#[derive(Debug, thiserror::Error)]
pub enum AikidoError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error {status}: {error} — {error_description}")]
    Api {
        status: u16,
        error: String,
        error_description: String,
    },

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Keychain error: {0}")]
    Keychain(String),

    #[error("{0}")]
    General(String),
}
