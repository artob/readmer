// This is free and unencumbered software released into the public domain.

use crate::Utf8Path;
use alloc::string::String;
use figment2::{
    Figment,
    providers::{Env, Format, Yaml},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    /// The project label.
    pub label: String,

    /// The project title.
    pub title: String,

    /// The project summary.
    pub summary: String,
}

impl Project {
    pub fn load(path: impl AsRef<Utf8Path>) -> Result<Self, figment2::Error> {
        Figment::new()
            .merge(Yaml::file(path.as_ref()))
            .merge(Env::prefixed("READMER_"))
            .extract()
    }

    pub fn to_json(&self) -> Value {
        self.clone().into_json()
    }

    pub fn into_json(self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}
