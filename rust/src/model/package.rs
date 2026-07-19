// This is free and unencumbered software released into the public domain.

use super::LoadError;
use crate::{Utf8Path, export};
use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
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

    /// The package version.
    pub version: String,

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
    pub licenses: Vec<String>,

    /// The package repository.
    pub repository: Option<String>,

    /// The package metadata, if any.
    pub metadata: Option<Value>,
}

impl Package {
    pub fn locate(dir_path: impl AsRef<Utf8Path>) -> Result<Self, LoadError> {
        let dir_path = dir_path.as_ref();
        for file_name in [
            #[cfg(feature = "rust")]
            "Cargo.toml",
            #[cfg(feature = "js")]
            "package.json",
            #[cfg(feature = "dart")]
            "pubspec.yaml",
            #[cfg(feature = "python")]
            "pyproject.toml",
            #[cfg(feature = "ruby")]
            ".gemspec.yaml", // TODO
        ] {
            let file_path = dir_path.join(file_name);
            if file_path.exists() {
                return Self::load(file_path);
            }
        }
        Err(LoadError::NoPackageFound(dir_path.into()))
    }

    pub fn load(file_path: impl AsRef<Utf8Path>) -> Result<Self, LoadError> {
        let file_path = file_path.as_ref();
        Ok(match file_path.file_name() {
            #[cfg(feature = "ruby")]
            Some(".gemspec.yaml") => export::ruby::load_gemspec(file_path)?.try_into()?, // TODO

            #[cfg(feature = "rust")]
            Some("Cargo.toml") => export::rust::load_cargo_toml(file_path)?.try_into()?,

            #[cfg(feature = "js")]
            Some("package.json") => export::js::load_package_json(file_path)?.try_into()?,

            #[cfg(feature = "dart")]
            Some("pubspec.yaml") => export::dart::load_pubspec(file_path)?.try_into()?,

            #[cfg(feature = "python")]
            Some("pyproject.toml") => export::python::load_pyproject_toml(file_path)?.try_into()?,

            _ => {
                return Err(LoadError::UnknownPackageFormat(file_path.into()));
            },
        })
    }

    pub fn to_json(&self) -> serde_json::Value {
        self.clone().into_json()
    }

    pub fn into_json(self) -> serde_json::Value {
        // Make sure to keep this in sync with `package.csv`!
        serde_json::json!({
            "name": self.name,
            "version": self.version,
            "author": &self.authors.first(),
            "authors": self.authors,
            "description": self.description,
            "homepage": self.homepage,
            "keywords": self.keywords,
            "categories": self.categories,
            "license": self.licenses.first(),
            "licenses": self.licenses,
            "repository": self.repository,
            "metadata": self.metadata,
        })
    }
}

#[cfg(feature = "dart")]
include!("package/dart.rs");

#[cfg(feature = "js")]
include!("package/js.rs");

#[cfg(feature = "python")]
include!("package/python.rs");

#[cfg(feature = "ruby")]
include!("package/ruby.rs");

#[cfg(feature = "rust")]
include!("package/rust.rs");
