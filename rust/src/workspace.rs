// This is free and unencumbered software released into the public domain.

use crate::{AncestorPath, Config, Git, GitError, RootedPath, Utf8Path, Utf8PathBuf};
use alloc::format;
use core::str::FromStr;
use std::io::{Error, ErrorKind, Result};

#[derive(Clone, Debug, Default)]
pub struct Workspace(pub(crate) RootedPath);

impl FromStr for Workspace {
    type Err = Error;

    fn from_str(input: &str) -> core::result::Result<Self, Self::Err> {
        let path = RootedPath::from_str(input)?;
        if let Some(up) = &path.up
            && !up.is_dir()
        {
            return Err(Error::from(ErrorKind::NotADirectory));
        }
        Ok(Self(path))
    }
}

impl AsRef<RootedPath> for Workspace {
    fn as_ref(&self) -> &RootedPath {
        &self.0
    }
}

impl Workspace {
    /// Locates the Git workspace containing the current directory.
    ///
    /// Uses the current directory as the workspace when Git is not installed or
    /// exits unsuccessfully (for example, outside a repository). No directories
    /// are created. Filesystem and process access require the `std` feature.
    ///
    /// # Errors
    ///
    /// Returns an error for other failures to execute Git, non-UTF-8 Git output,
    /// or a workspace prefix too deep to represent as an ancestor path.
    pub fn locate() -> Result<Self> {
        let git = Git::default();
        let down = match git.rev_parse_show_prefix() {
            Ok(down) => down,
            Err(GitError::CommandFailed { .. }) => return Ok(Self::default()),
            Err(GitError::IoError(error)) if error.kind() == ErrorKind::NotFound => {
                return Ok(Self::default());
            },
            Err(GitError::IoError(error)) => return Err(error),
            Err(GitError::InvalidUtf8(error)) => {
                return Err(Error::new(ErrorKind::InvalidData, error));
            },
        };
        let up_count = down.components().count();
        if up_count == 0 {
            return Ok(Self::default());
        }
        let up = Some(AncestorPath::try_from(up_count).map_err(|depth| {
            Error::new(
                ErrorKind::InvalidInput,
                format!("unsupported workspace ancestor depth: {depth}"),
            )
        })?);
        Ok(Self(RootedPath { up, down }))
    }

    pub fn path(&self) -> &RootedPath {
        &self.0
    }

    pub fn config(&self) -> Config {
        Config(self.config_path())
    }

    pub fn config_path(&self) -> Utf8PathBuf {
        self.join(".config/readmer")
    }

    pub fn join(&self, path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
        self.0.join(path)
    }
}
