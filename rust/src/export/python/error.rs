// This is free and unencumbered software released into the public domain.

use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadPyprojectError {
    #[error(transparent)]
    Other(#[from] Box<dyn core::error::Error>),
}

#[cfg(feature = "std")]
impl From<std::io::Error> for LoadPyprojectError {
    fn from(error: std::io::Error) -> Self {
        Self::Other(error.into())
    }
}

impl From<toml0::de::Error> for LoadPyprojectError {
    fn from(error: toml0::de::Error) -> Self {
        Self::Other(error.into())
    }
}
