// This is free and unencumbered software released into the public domain.

use crate::{Context, Engine, RenderError};
use alloc::{
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
};
use liquid::{
    Template,
    partials::{EagerCompiler, InMemorySource},
    reflection::ParserReflection,
};

type Partials = EagerCompiler<InMemorySource>;

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
        let template_data = self.0.get(&name).ok_or(RenderError::NotFound)?;
        let template = liquid::ParserBuilder::with_stdlib()
            .partials(partials())
            .build()?
            .parse(template_data)?;
        let context = liquid::to_object(&context.to_json())?;
        let output = template.render(&context)?;
        Ok(output) // always newline-terminated
    }
}

pub fn partials() -> Partials {
    let mut partials = Partials::empty();
    partials.add(
        "badge/unlicense.liquid",
        include_str!("../../data/badge/unlicense.md.liquid").trim_ascii_end(),
    );
    partials.add(
        "footer/share.liquid",
        include_str!("../../data/footer/share.md.liquid").trim_ascii_end(),
    );
    partials.add(
        "header/badges/dart.liquid",
        include_str!("../../data/header/badges/dart.md.liquid").trim_ascii_end(),
    );
    partials.add(
        "header/badges/js.liquid",
        include_str!("../../data/header/badges/js.md.liquid").trim_ascii_end(),
    );
    partials.add(
        "header/badges/python.liquid",
        include_str!("../../data/header/badges/python.md.liquid").trim_ascii_end(),
    );
    partials.add(
        "header/badges/ruby.liquid",
        include_str!("../../data/header/badges/ruby.md.liquid").trim_ascii_end(),
    );
    partials.add(
        "header/badges/rust.liquid",
        include_str!("../../data/header/badges/rust.md.liquid").trim_ascii_end(),
    );
    partials.add(
        "header/toc.liquid",
        include_str!("../../data/header/toc.md.liquid").trim_ascii_end(),
    );
    partials.add(
        "section/development.liquid",
        include_str!("../../data/section/development.md.liquid").trim_ascii_end(),
    );
    partials
}
