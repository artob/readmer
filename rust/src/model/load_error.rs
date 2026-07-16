// This is free and unencumbered software released into the public domain.

use crate::Utf8PathBuf;
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("no package found: {0}")]
    NoPackageFound(Utf8PathBuf),

    #[error("unknown package format: {0}")]
    UnknownPackageFormat(Utf8PathBuf),

    #[error(transparent)]
    Other(#[from] Box<dyn core::error::Error>),
}

#[cfg(feature = "rust")]
impl From<cargo_toml::Error> for LoadError {
    fn from(error: cargo_toml::Error) -> Self {
        LoadError::Other(Box::new(error))
    }
}
