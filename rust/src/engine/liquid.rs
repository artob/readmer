// This is free and unencumbered software released into the public domain.

use crate::{Context, Engine, RenderError, Workspace};
use alloc::{
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
    vec,
};
use liquid::{
    Template,
    partials::{EagerCompiler, InMemorySource, PartialSource},
    reflection::ParserReflection,
};

/// Liquid renderer with built-in, selected-project, and workspace partials.
///
/// Requires `liquid` and `std`. Missing partials allow fallback lookup; loading
/// or parsing failures in existing partials are returned with their cause.
#[derive(Clone, Debug)]
pub struct LiquidEngine {
    workspace: Workspace,
    templates: BTreeMap<String, String>,
}

impl LiquidEngine {
    pub fn new(workspace: Workspace) -> Self {
        Self {
            workspace,
            templates: BTreeMap::new(),
        }
    }
}

impl Engine for LiquidEngine {
    fn define_template(
        &mut self,
        name: String,
        data: String,
    ) -> Result<(), Box<dyn core::error::Error>> {
        self.templates.insert(name, data);
        Ok(())
    }

    fn render(&mut self, name: String, context: Box<dyn Context>) -> Result<String, RenderError> {
        let mut partials = stack::StackPartials::empty();
        partials.add(embed::EmbedSource::default());
        partials.add(file::FileSource::from_paths(vec![
            self.workspace.project_path(),
            self.workspace.join("."),
        ]));

        let template_data = self.templates.get(&name).ok_or(RenderError::NotFound)?;
        let template = liquid::ParserBuilder::with_stdlib()
            .partials(compiler::ErrorPreservingCompiler(partials))
            .build()?
            .parse(template_data)?;

        let context = liquid::to_object(&context.to_json())?;
        let mut output = template.render(&context)?;
        if output.contains("\n\n\n") {
            // Suppress spurious newlines due to empty partial output:
            output = output.replace("\n\n\n", "\n\n");
        }
        Ok(output) // always newline-terminated
    }
}

mod compiler;

mod embed;
pub use embed::*;

mod file;
pub use file::*;

mod stack;
pub use stack::*;
