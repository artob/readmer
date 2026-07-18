// This is free and unencumbered software released into the public domain.

use super::{Gemspec, LoadGemspecError};
use crate::Utf8Path;

pub fn load_gemspec(path: impl AsRef<Utf8Path>) -> Result<Gemspec, LoadGemspecError> {
    let input = std::fs::read_to_string(path.as_ref())?;
    let output = serde_norway::from_str(&input).unwrap(); // FIXME
    Ok(output)
}
