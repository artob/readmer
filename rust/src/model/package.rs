// This is free and unencumbered software released into the public domain.

use super::LoadError;
use crate::Utf8Path;
use alloc::{format, string::String, vec::Vec};
use figment2::{
    Figment,
    providers::{Env, Format, Yaml},
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Package {
    /// The package name.
    pub name: String,

    /// The package authors.
    pub authors: Vec<String>,

    /// The package title.
    pub description: Option<String>,

    /// The package summary.
    pub homepage: Option<String>,

    /// The package keywords.
    pub keywords: Vec<String>,

    /// The package categories.
    pub categories: Vec<String>,

    /// The package license.
    pub license: Option<String>,

    /// The package repository.
    pub repository: Option<String>,

    /// The package metadata, if any.
    pub metadata: Option<Value>,
}

impl Package {
    pub fn locate(dir_path: impl AsRef<Utf8Path>) -> Result<Self, LoadError> {
        let dir_path = dir_path.as_ref();
        let file_path = dir_path.join("Cargo.toml");
        if file_path.exists() {
            return Self::load(file_path);
        }
        Err(LoadError::NoPackageFound(dir_path.into()))
    }

    pub fn load(file_path: impl AsRef<Utf8Path>) -> Result<Self, LoadError> {
        let file_path = file_path.as_ref();
        Ok(match file_path.file_name() {
            #[cfg(feature = "rust")]
            Some(file_name) if file_name == "Cargo.toml" => {
                cargo_toml::Manifest::from_path(file_path)?.try_into()?
            },
            // TODO: Dart, JS, Python, Ruby
            _ => {
                return Err(LoadError::UnknownPackageFormat(file_path.into()));
            },
        })
    }

    pub fn to_json(&self) -> serde_json::Value {
        self.clone().into_json()
    }

    pub fn into_json(self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "authors": self.authors,
            "description": self.description,
            "homepage": self.homepage,
            "keywords": self.keywords,
            "categories": self.categories,
            "license": self.license,
            "repository": self.repository,
            "metadata": self.metadata,
        })
    }
}

#[cfg(feature = "rust")]
impl TryFrom<cargo_toml::Manifest> for Package {
    type Error = cargo_toml::Error;

    fn try_from(input: cargo_toml::Manifest) -> Result<Self, Self::Error> {
        use cargo_toml::Value;

        assert!(!input.needs_workspace_inheritance());
        let package = input.package.unwrap();
        Ok(Self {
            name: package.name,
            authors: package.authors.unwrap(),
            description: package.description.map(|x| x.unwrap()),
            homepage: package.homepage.map(|x| x.unwrap()),
            keywords: package.keywords.unwrap(),
            categories: package.categories.unwrap(),
            license: package.license.map(|x| x.unwrap()),
            repository: package.repository.map(|x| x.unwrap()),
            metadata: package.metadata.map(|x| x.try_into()).transpose()?,
        })
    }
}
