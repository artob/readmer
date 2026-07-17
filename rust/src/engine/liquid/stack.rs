// This is free and unencumbered software released into the public domain.

use crate::RootedPath;
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
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
use send_wrapper::SendWrapper;
use std::{
    fs::read_to_string,
    io::{Error, ErrorKind, Result},
};

pub type StackPartials = LazyCompiler<StackSource>;

#[derive(Debug)]
pub struct StackSource {
    sources: SendWrapper<Vec<Box<dyn PartialSource>>>,
}

impl Default for StackSource {
    fn default() -> Self {
        Self::new()
    }
}

impl StackSource {
    pub fn new() -> Self {
        Self {
            sources: send_wrapper::SendWrapper::new(Vec::new()),
        }
    }

    pub fn add(&mut self, source: impl PartialSource + 'static) {
        self.sources.push(Box::new(source))
    }
}

impl PartialSource for StackSource {
    fn contains(&self, name: &str) -> bool {
        self.sources.iter().any(|source| source.contains(name))
    }

    fn names<'a>(&'a self) -> Vec<&'a str> {
        Vec::new() // LazyCompiler doesn't call this method
    }

    fn try_get<'a>(&'a self, name: &str) -> Option<Cow<'a, str>> {
        for source in self.sources.iter() {
            if let Some(content) = source.try_get(name) {
                return Some(content);
            }
        }
        None
    }
}
