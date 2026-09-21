// This is free and unencumbered software released into the public domain.

//! Preserve errors from existing partials across Liquid's extension fallback.
//!
//! Liquid's `render` tag retries `name.liquid` on *any* store lookup error. Defer
//! errors for existing entries until rendering so only absent entries trigger
//! that retry. This also preserves syntax errors from the lazy compiler.

use alloc::{boxed::Box, sync::Arc, vec::Vec};
use liquid_core::{
    Error, Language, Result,
    partials::{PartialCompiler, PartialSource},
    runtime::{PartialStore, Renderable, Runtime},
};

pub(super) struct ErrorPreservingCompiler<C>(pub C);

impl<C: PartialCompiler> PartialCompiler for ErrorPreservingCompiler<C> {
    fn compile(self, language: Arc<Language>) -> Result<Box<dyn PartialStore + Send + Sync>> {
        Ok(Box::new(ErrorPreservingStore(self.0.compile(language)?)))
    }

    fn source(&self) -> &dyn PartialSource {
        self.0.source()
    }
}

#[derive(Debug)]
struct ErrorPreservingStore(Box<dyn PartialStore + Send + Sync>);

impl PartialStore for ErrorPreservingStore {
    fn contains(&self, name: &str) -> bool {
        self.0.contains(name)
    }

    fn names(&self) -> Vec<&str> {
        self.0.names()
    }

    fn try_get(&self, name: &str) -> Option<Arc<dyn Renderable>> {
        self.get(name).ok()
    }

    fn get(&self, name: &str) -> Result<Arc<dyn Renderable>> {
        // Check presence before loading so a file disappearing during the load
        // is still treated as a failure, not a reason to use another partial.
        let present = self.contains(name);
        match self.0.get(name) {
            Err(error) if present => Ok(Arc::new(FailedPartial(error))),
            result => result,
        }
    }
}

#[derive(Debug)]
struct FailedPartial(Error);

impl Renderable for FailedPartial {
    fn render_to(&self, _writer: &mut dyn std::io::Write, _runtime: &dyn Runtime) -> Result<()> {
        Err(self.0.clone())
    }
}
