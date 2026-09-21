// This is free and unencumbered software released into the public domain.

use crate::{
    Utf8Path, Utf8PathBuf,
    model::{LoadError, Project},
};
use alloc::string::String;
use std::{
    io::{Error, ErrorKind, Result},
    path::PathBuf,
};

#[derive(Clone, Debug)]
pub struct Config(pub(crate) Utf8PathBuf);

impl Config {
    /// Loads workspace project metadata and `READMER_` environment overrides.
    ///
    /// Requires `std`. Missing YAML is optional; see [`Project::load`].
    ///
    /// # Errors
    ///
    /// Returns [`LoadError::AtPath`] for unreadable or invalid metadata.
    #[cfg(feature = "std")]
    pub fn project(&self) -> core::result::Result<Project, LoadError> {
        Project::load(self.path_to_project_yaml())
    }

    /// Loads metadata for a workspace-relative subproject and environment overrides.
    ///
    /// Requires `std`. Missing YAML is optional; see [`Project::load`].
    ///
    /// # Errors
    ///
    /// Returns [`LoadError::AtPath`] for unreadable or invalid metadata.
    #[cfg(feature = "std")]
    pub fn subproject(
        &self,
        path: impl AsRef<Utf8Path>,
    ) -> core::result::Result<Project, LoadError> {
        Project::load(self.path_to_subproject_yaml(path))
    }

    pub fn has_template(&self, name: impl AsRef<str>) -> Result<bool> {
        self.has_file(name.as_ref())
    }

    pub fn read_template(&self, name: impl AsRef<str>) -> Result<String> {
        std::fs::read_to_string(self.join(name.as_ref()))
    }

    pub fn has_file(&self, name: impl AsRef<Utf8Path>) -> Result<bool> {
        self.join(name).try_exists()
    }

    pub(crate) fn path_to_project_yaml(&self) -> Utf8PathBuf {
        self.join("project.yaml")
    }

    pub(crate) fn path_to_subproject_yaml(&self, path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
        self.join(path).join("project.yaml")
    }

    pub fn join(&self, path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
        self.0.join(path)
    }
}
