// This is free and unencumbered software released into the public domain.

use crate::{Engine, RenderError};
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use liquid::Template;

#[derive(Clone, Debug)]
pub struct LiquidEngine;

impl Default for LiquidEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LiquidEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self) {}
}

impl Engine for LiquidEngine {}
