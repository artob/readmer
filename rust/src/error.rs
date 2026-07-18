// This is free and unencumbered software released into the public domain.

use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("template not found")]
    NotFound,

    #[error(transparent)]
    Other(#[from] Box<dyn core::error::Error>),
}

#[cfg(feature = "liquid")]
impl From<liquid::Error> for RenderError {
    fn from(error: liquid::Error) -> Self {
        Self::Other(Box::new(error))
    }
}

#[cfg(feature = "jinja2")]
impl From<minijinja::Error> for RenderError {
    fn from(error: minijinja::Error) -> Self {
        Self::Other(Box::new(error))
    }
}
