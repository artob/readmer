// This is free and unencumbered software released into the public domain.

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
