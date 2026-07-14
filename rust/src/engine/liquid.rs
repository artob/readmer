// This is free and unencumbered software released into the public domain.

use crate::{Engine, RenderError};
use alloc::{
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
};
use liquid::Template;

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

    fn render(&mut self, name: String) -> Result<String, RenderError> {
        let template_data = self.0.get(&name).ok_or(RenderError::NotFound)?;
        let template = liquid::ParserBuilder::with_stdlib()
            .build()?
            .parse(template_data)?;
        let output = template.render(&liquid::object!({}))?; // TODO
        Ok(output)
    }
}
