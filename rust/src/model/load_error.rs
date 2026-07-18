// This is free and unencumbered software released into the public domain.

use crate::{Utf8PathBuf, export};
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

#[cfg(feature = "python")]
impl From<export::python::LoadPyprojectError> for LoadError {
    fn from(error: export::python::LoadPyprojectError) -> Self {
        LoadError::Other(error.into())
    }
}

#[cfg(feature = "ruby")]
impl From<export::ruby::LoadGemspecError> for LoadError {
    fn from(error: export::ruby::LoadGemspecError) -> Self {
        LoadError::Other(error.into())
    }
}

#[cfg(feature = "rust")]
impl From<export::rust::LoadManifestError> for LoadError {
    fn from(error: export::rust::LoadManifestError) -> Self {
        LoadError::Other(error.into())
    }
}
