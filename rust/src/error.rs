// This is free and unencumbered software released into the public domain.

use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("not found")]
    NotFound,

    #[error(transparent)]
    Other(#[from] Box<dyn core::error::Error>),
}
