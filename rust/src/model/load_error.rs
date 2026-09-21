// This is free and unencumbered software released into the public domain.

use crate::Utf8PathBuf;
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use thiserror::Error;

/// An error encountered while finding or loading project or package metadata.
#[derive(Debug, Error)]
pub enum LoadError {
    /// No package was found in the directory or in the manifest at this path.
    ///
    /// A valid Cargo virtual workspace or tool-only Python manifest has no package.
    #[error("no package found: {0}")]
    NoPackageFound(Utf8PathBuf),

    /// The manifest filename is unrecognized or its language feature is disabled.
    #[error("unknown package format: {0}")]
    UnknownPackageFormat(Utf8PathBuf),

    /// Metadata could not be inspected, read, parsed, or converted at this path.
    #[error("failed to load `{path}`: {source}")]
    AtPath {
        /// The metadata file being loaded or inspected.
        path: Utf8PathBuf,
        /// The underlying filesystem, parser, or conversion failure.
        #[source]
        source: Box<dyn core::error::Error>,
    },

    /// An adapter failed to read, parse, resolve, or convert package metadata.
    #[error(transparent)]
    Other(#[from] Box<dyn core::error::Error>),
}

impl LoadError {
    pub(crate) fn at_path(
        path: impl Into<Utf8PathBuf>,
        source: impl core::error::Error + 'static,
    ) -> Self {
        Self::AtPath {
            path: path.into(),
            source: Box::new(source),
        }
    }
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
