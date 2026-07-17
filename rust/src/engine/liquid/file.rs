// This is free and unencumbered software released into the public domain.

use crate::RootedPath;
use alloc::{
    borrow::{Cow, ToOwned},
    string::{String, ToString},
    vec,
    vec::Vec,
};
use camino::Utf8PathBuf;
use liquid::{
    Template,
    partials::{EagerCompiler, LazyCompiler, PartialSource},
    reflection::ParserReflection,
};
use std::{
    fs::read_to_string,
    io::{Error, ErrorKind, Result},
};

pub type FilePartials = LazyCompiler<FileSource>;

#[derive(Clone, Debug, Default)]
pub struct FileSource {
    base: RootedPath,
}

impl FileSource {
    pub fn new(base: RootedPath) -> Self {
        Self { base }
    }

    fn load(&self, name: &str) -> Result<String> {
        let path = self.join(name);
        if !path.try_exists()? {
            return Err(ErrorKind::NotFound.into());
        }
        let mut result = match path.extension() {
            Some("liquid") | Some("md") => read_to_string(path)?,
            Some("json") => todo!(),                            // TODO
            Some("csv") | Some("tsv") => todo!(),               // TODO
            _ => return Err(ErrorKind::InvalidFilename.into()), // Rust 1.87+
        };
        result.truncate(result.trim_ascii_end().len());
        Ok(result)
    }

    fn join(&self, name: &str) -> Utf8PathBuf {
        self.base.join(name)
    }
}

impl PartialSource for FileSource {
    fn contains(&self, name: &str) -> bool {
        self.join(name).exists()
    }

    fn names<'a>(&'a self) -> Vec<&'a str> {
        Vec::new() // LazyCompiler doesn't call this method
    }

    fn try_get<'a>(&'a self, name: &str) -> Option<Cow<'a, str>> {
        self.load(name).ok().map(Cow::Owned)
    }
}
