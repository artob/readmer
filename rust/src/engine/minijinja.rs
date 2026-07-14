// This is free and unencumbered software released into the public domain.

use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use clientele::crates::camino::{Utf8Path, Utf8PathBuf};
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

    #[cfg(feature = "std")]
    pub fn load_template(
        &mut self,
        name: impl Into<String>,
        path: impl Into<Utf8PathBuf>,
    ) -> Result<(), Box<dyn core::error::Error>> {
        let name = name.into();
        let path = path.into();
        let env = &mut self.0;
        let content = std::fs::read_to_string(&path)?;
        env.add_template_owned(name, content)?;
        Ok(())
    }

    pub fn add_template(
        &mut self,
        name: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<(), Error> {
        let env = &mut self.0;
        env.add_template_owned(name.into(), content.into())
    }

    pub fn render(&mut self, name: impl Into<String>) -> Result<String, Error> {
        let env = &mut self.0;
        let name: String = name.into();
        let template = env.get_template(name.as_ref())?;
        let result = template.render(&minijinja::context!())?; // TODO
        Ok(result)
    }
}
