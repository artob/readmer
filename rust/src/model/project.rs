// This is free and unencumbered software released into the public domain.

use crate::{Utf8Path, model::LoadError};
use alloc::{string::String, vec::Vec};
use figment2::{
    Figment,
    providers::{Env, Format, Yaml},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    /// The project name.
    pub name: Option<String>,

    /// The project label.
    pub label: Option<String>,

    /// The project title.
    pub title: Option<String>,

    /// The project summary.
    pub summary: Option<String>,

    /// The project links.
    pub links: Option<Vec<String>>,
}

impl Project {
    /// Loads optional YAML metadata at exactly `path`, then applies `READMER_`
    /// environment overrides. Requires `std`.
    ///
    /// A missing file is treated as empty metadata, allowing environment-only
    /// configuration. Ancestor directories are never searched. An existing but
    /// unreadable file, including a dangling symlink, is an error.
    ///
    /// # Errors
    ///
    /// Returns [`LoadError::AtPath`] with the path and underlying error for
    /// filesystem failures, invalid YAML, or invalid metadata/environment types.
    #[cfg(feature = "std")]
    pub fn load(path: impl AsRef<Utf8Path>) -> Result<Self, LoadError> {
        let path = path.as_ref();
        let mut figment = Figment::new();
        match std::fs::symlink_metadata(path) {
            Ok(_) => {
                let contents = std::fs::read_to_string(path)
                    .map_err(|error| LoadError::at_path(path, error))?;
                figment = figment.merge(Yaml::string(&contents));
            },
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {},
            Err(error) => return Err(LoadError::at_path(path, error)),
        }
        figment
            .merge(Env::prefixed("READMER_"))
            .extract()
            .map_err(|error| LoadError::at_path(path, error))
    }

    pub fn to_json(&self) -> Value {
        self.clone().into_json()
    }

    pub fn into_json(self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}
