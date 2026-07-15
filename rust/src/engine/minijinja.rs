// This is free and unencumbered software released into the public domain.

use crate::{Engine, RenderError, Utf8Path, Utf8PathBuf};
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use minijinja::{Environment, Error, UndefinedBehavior};

#[derive(Clone, Debug)]
pub struct MinijinjaEngine(Environment<'static>);

impl Default for MinijinjaEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MinijinjaEngine {
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.set_undefined_behavior(UndefinedBehavior::Strict);
        Self(env)
    }
}

impl Engine for MinijinjaEngine {
    fn define_template(
        &mut self,
        name: String,
        data: String,
    ) -> Result<(), Box<dyn core::error::Error>> {
        let env = &mut self.0;
        env.add_template_owned(name, data)?;
        Ok(())
    }

    fn render(&mut self, name: String, context: serde_json::Value) -> Result<String, RenderError> {
        let env = &mut self.0;
        let template = env.get_template(name.as_ref())?;
        let output = template.render(context)?;
        Ok(output)
    }
}
