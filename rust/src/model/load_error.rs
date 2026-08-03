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

#[cfg(feature = "dart")]
impl From<distrib::dart::LoadPubspecError> for LoadError {
    fn from(error: distrib::dart::LoadPubspecError) -> Self {
        LoadError::Other(error.into())
    }
}

#[cfg(feature = "gleam")]
impl From<distrib::gleam::LoadPackageError> for LoadError {
    fn from(error: distrib::gleam::LoadPackageError) -> Self {
        LoadError::Other(error.into())
    }
}

#[cfg(feature = "js")]
impl From<distrib::js::LoadPackageError> for LoadError {
    fn from(error: distrib::js::LoadPackageError) -> Self {
        LoadError::Other(error.into())
    }
}

#[cfg(feature = "python")]
impl From<distrib::python::LoadPyprojectError> for LoadError {
    fn from(error: distrib::python::LoadPyprojectError) -> Self {
        LoadError::Other(error.into())
    }
}

#[cfg(feature = "ruby")]
impl From<distrib::ruby::LoadGemspecError> for LoadError {
    fn from(error: distrib::ruby::LoadGemspecError) -> Self {
        LoadError::Other(error.into())
    }
}

#[cfg(feature = "rust")]
impl From<distrib::rust::LoadManifestError> for LoadError {
    fn from(error: distrib::rust::LoadManifestError) -> Self {
        LoadError::Other(error.into())
    }
}
