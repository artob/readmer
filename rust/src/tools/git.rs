// This is free and unencumbered software released into the public domain.

use crate::{Utf8Path, Utf8PathBuf};
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

#[cfg(feature = "std")]
impl Git {
    /// Selects the Git executable path. Requires `std`.
    pub fn at(path: impl Into<Utf8PathBuf>) -> Self {
        Self(path.into())
    }

    /// Reads the `origin` remote in the current directory. Requires `std`.
    /// Returns [`GitError`] if Git fails or its output is not UTF-8.
    pub fn remote_get_url(&self) -> Result<String, GitError> {
        self.execute(["remote", "get-url", "origin"])
    }

    /// Reads the Git worktree root for the current directory. Requires `std`.
    /// Returns [`GitError`] if Git fails or its output is not UTF-8.
    pub fn rev_parse_show_toplevel(&self) -> Result<Utf8PathBuf, GitError> {
        self.rev_parse("--show-toplevel")
    }

    /// Reads the current directory's prefix within its Git worktree. Requires `std`.
    /// Returns [`GitError`] if Git fails or its output is not UTF-8.
    pub fn rev_parse_show_prefix(&self) -> Result<Utf8PathBuf, GitError> {
        self.rev_parse("--show-prefix")
    }

    /// Runs `git rev-parse` with one option in the current directory. Requires `std`.
    /// Returns [`GitError`] if Git fails or its output is not UTF-8.
    pub fn rev_parse(&self, option: &str) -> Result<Utf8PathBuf, GitError> {
        self.execute(["rev-parse", option]).map(|s| s.into())
    }

    /// Executes Git in the current directory, removing its final line ending.
    /// Requires `std`; errors are as for [`Self::execute_in`].
    pub fn execute(
        &self,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> Result<String, GitError> {
        self.execute_in(".", args)
    }

    /// Executes Git in `directory` without changing the process's current directory.
    /// Requires `std`. Only the final line ending is removed from stdout, preserving
    /// spaces in directory names and other Git output.
    ///
    /// # Errors
    ///
    /// Returns [`GitError`] for process/directory access failures, nonzero exit
    /// status, or stdout that is not UTF-8.
    pub fn execute_in(
        &self,
        directory: impl AsRef<Utf8Path>,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> Result<String, GitError> {
        let output = Command::new(&self.0)
            .current_dir(directory.as_ref())
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
        if stdout.ends_with('\n') {
            stdout.pop();
            #[cfg(windows)]
            if stdout.ends_with('\r') {
                stdout.pop();
            }
        }
        Ok(stdout)
    }
}
