// This is free and unencumbered software released into the public domain.

use crate::{Config, Git, GitError, RootedPath, Utf8Path, Utf8PathBuf};
use core::str::FromStr;
use std::io::{Error, ErrorKind, Result};

/// An ancestor workspace root and a selected project contained within it.
///
/// The root must contain both the invocation directory and the selected project.
/// [`Self::path`] retains the invocation directory's ancestor-relative relationship;
/// canonical filesystem anchoring and the selected project's prefix are stored
/// separately. The default targets the current directory without filesystem access.
/// Relative constructor arguments are relative to the caller, not to each other.
/// Filesystem-based resolution requires `std`.
#[derive(Clone, Debug, Default)]
pub struct Workspace {
    path: RootedPath,
    root: Option<Utf8PathBuf>,
    project_prefix: Utf8PathBuf,
}

#[cfg(feature = "std")]
impl FromStr for Workspace {
    type Err = Error;

    fn from_str(input: &str) -> core::result::Result<Self, Self::Err> {
        Self::new(input, ".")
    }
}

impl AsRef<RootedPath> for Workspace {
    fn as_ref(&self) -> &RootedPath {
        &self.path
    }
}

impl Workspace {
    /// Selects `project` within an explicit workspace `root`.
    ///
    /// Both paths are resolved relative to the caller's current directory;
    /// `.`/`..` and symlinks are resolved. The root must be the caller's current
    /// directory or an ancestor. Requires `std`.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid directories, a root that is neither the current
    /// directory nor an ancestor, or a project outside the root.
    ///
    /// # Examples
    ///
    /// ```
    /// let workspace = readmer::Workspace::new(".", ".")?;
    /// assert!(workspace.project_prefix().as_str().is_empty());
    /// # Ok::<(), std::io::Error>(())
    /// ```
    #[cfg(feature = "std")]
    pub fn new(root: impl AsRef<Utf8Path>, project: impl AsRef<Utf8Path>) -> Result<Self> {
        let root = RootedPath::canonical_directory(root.as_ref())?;
        let path = root.as_str().parse()?;
        let project = RootedPath::canonical_directory(project.as_ref())?;
        let project_prefix = project
            .strip_prefix(&root)
            .map_err(|_| {
                Error::new(
                    ErrorKind::InvalidInput,
                    alloc::format!("project `{project}` is outside workspace `{root}`"),
                )
            })?
            .to_path_buf();
        Ok(Self {
            path,
            root: Some(root),
            project_prefix,
        })
    }

    /// Locates the Git workspace containing the current directory.
    ///
    /// Uses the current directory as the workspace when Git is not installed or
    /// exits unsuccessfully (for example, outside a repository). No directories
    /// are created. Filesystem and process access require the `std` feature.
    ///
    /// # Errors
    ///
    /// Returns an error for other failures to execute Git, non-UTF-8 Git output,
    /// or an invalid current directory. See [`Self::locate_from`].
    #[cfg(feature = "std")]
    pub fn locate() -> Result<Self> {
        Self::locate_from(".")
    }

    /// Selects `project` within the workspace discovered from the current directory.
    /// Falls back to the current directory if Git is unavailable or exits
    /// unsuccessfully. Requires `std`.
    ///
    /// Project selection never changes the discovery origin or establishes a
    /// workspace below the caller. Relative project paths are caller-relative.
    ///
    /// # Errors
    ///
    /// Returns errors for invalid project/root directories, other Git execution
    /// failures, non-UTF-8 output, or a root that does not contain both the current
    /// directory and the project.
    #[cfg(feature = "std")]
    pub fn locate_from(project: impl AsRef<Utf8Path>) -> Result<Self> {
        let current = RootedPath::canonical_directory(Utf8Path::new("."))?;
        let git = Git::default();
        let root = match git.execute_in(&current, ["rev-parse", "--show-toplevel"]) {
            Ok(root) => Utf8PathBuf::from(root),
            Err(GitError::CommandFailed { .. }) => current.clone(),
            Err(GitError::IoError(error)) if error.kind() == ErrorKind::NotFound => current.clone(),
            Err(GitError::IoError(error)) => return Err(error),
            Err(GitError::InvalidUtf8(error)) => {
                return Err(Error::new(ErrorKind::InvalidData, error));
            },
        };
        Self::new(root, project)
    }

    /// Returns the ancestor-relative paths between the invocation directory and
    /// the root. Use [`Self::project_prefix`] for the selected project's prefix.
    pub fn path(&self) -> &RootedPath {
        &self.path
    }

    /// Returns the selected project's directory.
    pub fn project_path(&self) -> Utf8PathBuf {
        if self.project_prefix.as_str().is_empty() {
            self.root.clone().unwrap_or_else(|| ".".into())
        } else {
            self.join(&self.project_prefix)
        }
    }

    /// Returns the selected project's path relative to the workspace root.
    pub fn project_prefix(&self) -> &Utf8Path {
        &self.project_prefix
    }

    /// Returns configuration rooted at the workspace's `.config/readmer/`.
    pub fn config(&self) -> Config {
        Config(self.config_path())
    }

    /// Returns the workspace's shared `.config/readmer/` directory.
    pub fn config_path(&self) -> Utf8PathBuf {
        self.join(".config/readmer")
    }

    /// Returns the selected project's directory within `.config/readmer/`.
    /// At the workspace root this is the shared configuration directory itself.
    pub fn project_config_path(&self) -> Utf8PathBuf {
        self.config_path().join(self.project_prefix())
    }

    /// Joins a path to the workspace root, without applying the project prefix.
    pub fn join(&self, path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
        match &self.root {
            Some(root) => root.join(path),
            None => self.path.join(path),
        }
    }
}
