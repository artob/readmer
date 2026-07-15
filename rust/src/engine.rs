// This is free and unencumbered software released into the public domain.

use crate::{Context, RenderError, Utf8PathBuf};
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use core::error::Error;
use dyn_clone::DynClone;

pub trait Engine: DynClone {
    #[cfg(feature = "std")]
    fn load_template(
        &mut self,
        name: String,
        path: Utf8PathBuf,
    ) -> Result<(), Box<dyn core::error::Error>> {
        let content = std::fs::read_to_string(&path)?;
        self.define_template(name, content)
    }

    fn define_template(
        &mut self,
        _name: String,
        _data: String,
    ) -> Result<(), Box<dyn core::error::Error>> {
        Ok(())
    }

    fn render(&mut self, _name: String, _context: Context) -> Result<String, RenderError> {
        Err(RenderError::NotFound)
    }
}

dyn_clone::clone_trait_object!(Engine);

#[cfg(feature = "liquid")]
mod liquid;
#[cfg(feature = "liquid")]
pub use liquid::*;

#[cfg(feature = "jinja2")]
mod minijinja;
#[cfg(feature = "jinja2")]
pub use minijinja::*;
