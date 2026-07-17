// This is free and unencumbered software released into the public domain.

use crate::{Context, Engine, RenderError};
use alloc::{
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
};
use liquid::{
    Template,
    partials::{EagerCompiler, InMemorySource, PartialSource},
    reflection::ParserReflection,
};

#[derive(Clone, Debug)]
pub struct LiquidEngine(BTreeMap<String, String>);

impl Default for LiquidEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LiquidEngine {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn render(&self) {}
}

impl Engine for LiquidEngine {
    fn define_template(
        &mut self,
        name: String,
        data: String,
    ) -> Result<(), Box<dyn core::error::Error>> {
        self.0.insert(name, data);
        Ok(())
    }

    fn render(&mut self, name: String, context: Box<dyn Context>) -> Result<String, RenderError> {
        let mut partials = stack::StackPartials::empty();
        partials.add(embed::EmbedSource::default());
        partials.add(file::FileSource::default());
        let template_data = self.0.get(&name).ok_or(RenderError::NotFound)?;
        let template = liquid::ParserBuilder::with_stdlib()
            .partials(partials)
            .build()?
            .parse(template_data)?;
        let context = liquid::to_object(&context.to_json())?;
        let output = template.render(&context)?;
        Ok(output) // always newline-terminated
    }
}

mod embed;
pub use embed::*;

mod file;
pub use file::*;

mod stack;
pub use stack::*;
