// This is free and unencumbered software released into the public domain.

use crate::{AncestorPath, Config, Git, RootedPath, Utf8Path, Utf8PathBuf};
use alloc::string::String;
use core::str::FromStr;
use dogma::FromPathError;
use std::{
    io::{Error, ErrorKind, Result},
    path::PathBuf,
};

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
    pub fn locate() -> Result<Self> {
        let git = Git::default();
        let down = git.rev_parse_show_prefix().unwrap(); // FIXME
        let up_count = down.components().count();
        if up_count == 0 {
            return Ok(Self::default());
        }
        let up = AncestorPath::try_from(up_count).ok();
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
