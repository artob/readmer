// This is free and unencumbered software released into the public domain.

use crate::{Context, Engine, RenderError, Utf8Path, Utf8PathBuf, Workspace};
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use minijinja::{Environment, Error, UndefinedBehavior};

#[derive(Clone, Debug)]
pub struct MinijinjaEngine {
    #[allow(unused)]
    workspace: Workspace,
    environment: Environment<'static>,
}

impl MinijinjaEngine {
    pub fn new(workspace: Workspace) -> Self {
        let mut environment = Environment::new();
        environment.set_undefined_behavior(UndefinedBehavior::Strict);
        Self {
            workspace,
            environment,
        }
    }
}

impl Engine for MinijinjaEngine {
    fn define_template(
        &mut self,
        name: String,
        data: String,
    ) -> Result<(), Box<dyn core::error::Error>> {
        self.environment.add_template_owned(name, data)?;
        Ok(())
    }

    fn render(&mut self, name: String, context: Box<dyn Context>) -> Result<String, RenderError> {
        let template = self.environment.get_template(name.as_ref())?;
        let mut output = template.render(context.to_json())?;
        output.push('\n'); // ensure newline-termination
        Ok(output)
    }
}
