// This is free and unencumbered software released into the public domain.

use crate::{AncestorPath, FromPathError, Utf8PathBuf};
use alloc::vec::Vec;
use camino::{FromPathBufError, Utf8Components, Utf8Path};
use core::str::FromStr;

/// A relative path within the workspace.
pub type WorkspacePath = RootedPath;

/// A rooted path is a path relative to a particular root directory.
///
/// Note that what's meant by the root directory here is *not* the file system
/// root directory, but rather just a root directory of your choice.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RootedPath {
    /// The path from the current directory to the root directory.
    pub up: Option<AncestorPath>,

    /// The path from the root directory to the current directory.
    pub down: Utf8PathBuf,
}

impl FromStr for RootedPath {
    type Err = std::io::Error;

    fn from_str(input: &str) -> core::result::Result<Self, Self::Err> {
        use alloc::format;
        use std::io::{Error, ErrorKind};
        let up = AncestorPath::from_str(input).map_err(FromPathError::into_io_error)?;
        let cwd = Utf8PathBuf::try_from(std::env::current_dir()?)
            .map_err(FromPathBufError::into_io_error)?;
        let mut parts = cwd.components().rev().take(up.depth()).collect::<Vec<_>>();
        parts.reverse();
        if parts.len() != up.depth() {
            return Err(Error::new(
                ErrorKind::Other,
                format!("path depth mismatch: {} != {}", parts.len(), up.depth()),
            ));
        }
        let down = Utf8PathBuf::from_iter(parts);
        Ok(Self { up: Some(up), down })
    }
}

impl RootedPath {
    pub fn join(&self, path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
        match self.up {
            Some(ref up) => up.to_path_buf().join(path),
            None => path.as_ref().to_path_buf(),
        }
    }
}
