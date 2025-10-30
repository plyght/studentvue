use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("XML parsing failed: {0}")]
    XmlParse(String),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Decode error: {0}")]
    Decode(#[from] base64::DecodeError),
}

pub type Result<T> = std::result::Result<T, Error>;
