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

    pub fn add_template(
        &mut self,
        name: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<(), Error> {
        let env = &mut self.0;
        env.add_template_owned(name.into(), content.into())
    }
}

impl Engine for MinijinjaEngine {
    #[cfg(feature = "std")]
    fn load_template(
        &mut self,
        name: &str,
        path: &Utf8PathBuf,
    ) -> Result<(), Box<dyn core::error::Error>> {
        let name: String = name.into();
        let path: Utf8PathBuf = path.clone();
        let env = &mut self.0;
        let content = std::fs::read_to_string(&path)?;
        env.add_template_owned(name, content)?;
        Ok(())
    }

    fn render(&mut self, name: &str) -> Result<String, RenderError> {
        let env = &mut self.0;
        let name: String = name.into();
        let template = env
            .get_template(name.as_ref())
            .map_err(|e| RenderError::Other(Box::new(e)))?;
        let result = template
            .render(&minijinja::context!())
            .map_err(|e| RenderError::Other(Box::new(e)))?;
        Ok(result)
    }
}
