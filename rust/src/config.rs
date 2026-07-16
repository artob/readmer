// This is free and unencumbered software released into the public domain.

use crate::{Utf8Path, Utf8PathBuf, model::Project};
use alloc::string::String;
use std::{
    io::{Error, ErrorKind, Result},
    path::PathBuf,
};

#[derive(Clone, Debug)]
pub struct Config(pub(crate) Utf8PathBuf);

impl Config {
    pub fn project(&self) -> Option<Project> {
        Project::load(self.path_to_project_yaml()).ok()
    }

    pub fn subproject(&self, path: impl AsRef<Utf8Path>) -> Option<Project> {
        Project::load(self.path_to_subproject_yaml(path)).ok()
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
