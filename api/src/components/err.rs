use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ApiKeyError {
    Invalid,
    Missing,
}

impl Display for ApiKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiKeyError::Invalid => write!(f, "Invalid API key"),
            ApiKeyError::Missing => write!(f, "No API key provided"),
        }
    }
}

impl Error for ApiKeyError {}
