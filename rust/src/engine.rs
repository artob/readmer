// This is free and unencumbered software released into the public domain.

use crate::{RenderError, Utf8PathBuf};
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use core::error::Error;
use dyn_clone::DynClone;

pub trait Engine: DynClone {
    fn load_template(
        &mut self,
        _name: &str,
        _path: &Utf8PathBuf,
    ) -> Result<(), Box<dyn core::error::Error>> {
        Err(RenderError::NotFound.into())
    }

    fn render(&mut self, _name: &str) -> Result<String, RenderError> {
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
