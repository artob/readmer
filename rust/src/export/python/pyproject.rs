// This is free and unencumbered software released into the public domain.

use super::LoadPyprojectError;
use crate::Utf8Path;

pub use pyproject_toml::{Contact, License};

pub type PyprojectToml = pyproject_toml::PyProjectToml;

pub fn load_pyproject_toml(
    path: impl AsRef<Utf8Path>,
) -> Result<PyprojectToml, LoadPyprojectError> {
    let input = std::fs::read_to_string(path.as_ref())?;
    Ok(pyproject_toml::PyProjectToml::new(&input)?)
}
