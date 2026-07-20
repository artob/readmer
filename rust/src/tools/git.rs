// This is free and unencumbered software released into the public domain.

use crate::Utf8PathBuf;
use alloc::string::{FromUtf8Error, String};
use std::{ffi::OsStr, io::Error, process::Command};

/// An error that can occur when running Git commands.
#[derive(Debug)]
pub enum GitError {
    /// Git is not installed or couldn't be executed.
    IoError(Error),
    /// The command ran but returned a non-zero exit status (e.g., not a Git repo).
    CommandFailed { code: Option<i32>, stderr: String },
    /// The output from Git was not valid UTF-8.
    InvalidUtf8(FromUtf8Error),
}

#[derive(Clone, Debug)]
pub struct Git(Utf8PathBuf);

impl Default for Git {
    fn default() -> Self {
        Self("git".into())
    }
}

impl Git {
    pub fn at(path: impl Into<Utf8PathBuf>) -> Self {
        Self(path.into())
    }

    pub fn remote_get_url(&self) -> Result<String, GitError> {
        self.execute(["remote", "get-url", "origin"])
    }

    pub fn rev_parse_show_toplevel(&self) -> Result<Utf8PathBuf, GitError> {
        self.rev_parse("--show-toplevel")
    }

    pub fn rev_parse_show_prefix(&self) -> Result<Utf8PathBuf, GitError> {
        self.rev_parse("--show-prefix")
    }

    pub fn rev_parse(&self, option: &str) -> Result<Utf8PathBuf, GitError> {
        self.execute(["rev-parse", option]).map(|s| s.into())
    }

    pub fn execute(
        &self,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> Result<String, GitError> {
        let output = Command::new(&self.0)
            .args(args)
            .output()
            .map_err(GitError::IoError)?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
            return Err(GitError::CommandFailed {
                code: output.status.code(),
                stderr,
            });
        }
        let mut stdout = String::from_utf8(output.stdout).map_err(GitError::InvalidUtf8)?;
        stdout.truncate(stdout.trim_ascii_end().len());
        Ok(stdout)
    }
}
