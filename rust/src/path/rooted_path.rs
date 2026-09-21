// This is free and unencumbered software released into the public domain.

use crate::{Utf8Path, Utf8PathBuf};
use alloc::format;
use camino::FromPathBufError;
use core::str::FromStr;
use dogma::AncestorPath;

/// A relative path within the workspace.
pub type WorkspacePath = RootedPath;

/// A rooted path is a path relative to a particular root directory.
///
/// Note that what's meant by the root directory here is *not* the file system
/// root directory, but rather just a root directory of your choice.
/// The root must be the current directory or an ancestor. This value stores
/// relative paths in both directions, not an absolute filesystem anchor.
/// Parsing requires `std` and accepts relative or absolute directory paths,
/// resolving symlinks before checking the ancestor relationship.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RootedPath {
    /// The path from the current directory to the root directory.
    /// `None` means the current directory is the root.
    pub up: Option<AncestorPath>,

    /// The path from the root directory to the current directory.
    pub down: Utf8PathBuf,
}

#[cfg(feature = "std")]
impl FromStr for RootedPath {
    type Err = std::io::Error;

    fn from_str(input: &str) -> core::result::Result<Self, Self::Err> {
        use std::io::{Error, ErrorKind};
        let root = Self::canonical_directory(Utf8Path::new(input))?;
        let current = Self::canonical_directory(Utf8Path::new("."))?;
        let down = current
            .strip_prefix(&root)
            .map_err(|_| {
                Error::new(
                    ErrorKind::InvalidInput,
                    format!("workspace `{root}` must be the current directory or an ancestor of `{current}`"),
                )
            })?
            .to_path_buf();
        let up = match down.components().count() {
            0 => None,
            depth => Some(AncestorPath::try_from(depth).map_err(|depth| {
                Error::new(
                    ErrorKind::InvalidInput,
                    format!("unsupported workspace ancestor depth: {depth}"),
                )
            })?),
        };
        Ok(Self { up, down })
    }
}

impl RootedPath {
    #[cfg(feature = "std")]
    pub(crate) fn canonical_directory(path: &Utf8Path) -> std::io::Result<Utf8PathBuf> {
        use std::io::{Error, ErrorKind};
        if path.as_str().is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "directory path must not be empty",
            ));
        }
        let resolved = std::fs::canonicalize(path).map_err(|error| {
            Error::new(
                error.kind(),
                format!("cannot resolve directory `{path}`: {error}"),
            )
        })?;
        let resolved = Utf8PathBuf::try_from(resolved).map_err(FromPathBufError::into_io_error)?;
        if !std::fs::metadata(&resolved)?.is_dir() {
            return Err(Error::new(
                ErrorKind::NotADirectory,
                format!("not a directory: `{path}`"),
            ));
        }
        Ok(resolved)
    }

    /// Joins a path to this root, returning a path relative to the current directory.
    /// An absolute argument replaces the root.
    /// The downward path in [`Self::down`] is not applied.
    pub fn join(&self, path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
        match self.up {
            Some(ref up) => up.to_path_buf().join(path),
            None => path.as_ref().to_path_buf(),
        }
    }
}
