// This is free and unencumbered software released into the public domain.

use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadManifestError {
    #[error(transparent)]
    Other(#[from] Box<dyn core::error::Error>),
}

#[cfg(feature = "std")]
impl From<std::io::Error> for LoadManifestError {
    fn from(error: std::io::Error) -> Self {
        Self::Other(error.into())
    }
}

impl From<toml1::de::Error> for LoadManifestError {
    fn from(error: toml1::de::Error) -> Self {
        Self::Other(error.into())
    }
}

impl From<cargo_toml::Error> for LoadManifestError {
    fn from(error: cargo_toml::Error) -> Self {
        Self::Other(error.into())
    }
}
