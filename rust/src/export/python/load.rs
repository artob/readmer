// This is free and unencumbered software released into the public domain.

use super::{Contact, License, LoadPyprojectError, PyprojectToml};
use crate::Utf8Path;

pub fn load_pyproject_toml(
    path: impl AsRef<Utf8Path>,
) -> Result<PyprojectToml, LoadPyprojectError> {
    let input = std::fs::read_to_string(path.as_ref())?;
    Ok(pyproject_toml::PyProjectToml::new(&input)?)
}
