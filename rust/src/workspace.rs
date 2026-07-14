// This is free and unencumbered software released into the public domain.

use crate::{Git, Utf8PathBuf};
use alloc::string::String;
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
    pub fn locate() -> Result<(Self, Option<Utf8PathBuf>)> {
        let git = Git::default();
        let prefix = git.show_prefix().unwrap(); // FIXME
        let parent_count = prefix.components().count();
        if parent_count == 0 {
            return Ok((Self(".".into()), None));
        }
        let mut path = Utf8PathBuf::new();
        for _ in 0..parent_count {
            path.push("..");
        }
        Ok((Self(path), Some(prefix)))
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
