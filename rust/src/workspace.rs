// This is free and unencumbered software released into the public domain.

use alloc::string::String;
use clientele::crates::camino::Utf8PathBuf;
use core::str::FromStr;
use std::{
    io::{Error, ErrorKind, Result},
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct Workspace(pub(crate) Utf8PathBuf);

impl Default for Workspace {
    fn default() -> Self {
        Self(".".into())
    }
}

impl FromStr for Workspace {
    type Err = Error;

    fn from_str(input: &str) -> core::result::Result<Self, Self::Err> {
        let path: Utf8PathBuf = input.into();
        if !path.is_dir() {
            return Err(Error::from(ErrorKind::NotADirectory));
        }
        Ok(Self(path))
    }
}

impl Workspace {
    pub fn locate() -> Result<Self> {
        Self::locate_from(".")
    }

    pub fn locate_from(subpath: impl Into<Utf8PathBuf>) -> Result<Self> {
        let subpath = subpath.into();
        if !subpath.is_dir() {
            return Err(Error::from(ErrorKind::NotADirectory));
        }
        // TODO: traverse up to find the workspace root
        Ok(Self(subpath))
    }

    pub fn has_template(&self, name: impl AsRef<str>) -> Result<bool> {
        self.template_path(name.as_ref()).try_exists()
    }

    pub fn read_template(&self, name: impl AsRef<str>) -> Result<String> {
        std::fs::read_to_string(self.template_path(name.as_ref()))
    }

    pub fn template_path(&self, name: impl AsRef<str>) -> Utf8PathBuf {
        self.config_path().join(name.as_ref())
    }

    pub fn config_path(&self) -> Utf8PathBuf {
        self.join(".config/readmer")
    }

    pub(crate) fn join(&self, path: impl AsRef<str>) -> Utf8PathBuf {
        self.0.join(path.as_ref())
    }
}
