// This is free and unencumbered software released into the public domain.

use super::LoadError;
use crate::Utf8Path;
use alloc::{
    borrow::Cow,
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
pub struct Language {
    pub name: Cow<'static, str>,

    pub label: Cow<'static, str>,

    pub extensions: Vec<Cow<'static, str>>,

    pub version: Option<String>,

    pub minimum_version: Option<String>,

    pub maximum_version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Package {
    /// The primary language.
    pub language: Language,

    /// The implementation languages.
    pub languages: Vec<Language>,

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
    /// Loads the first package found in a directory using enabled language adapters.
    ///
    /// Detection checks Gleam, JavaScript, Dart, Python, Ruby, then Rust. Manifests
    /// without a package section (such as tool-only Python manifests and Cargo
    /// virtual workspaces) are skipped. Filesystem access requires `std`.
    ///
    /// # Errors
    ///
    /// Returns [`LoadError::NoPackageFound`] if no package is found, or propagates
    /// [`LoadError::AtPath`] for a manifest that cannot be inspected, read, or parsed.
    #[cfg(feature = "std")]
    pub fn locate(dir_path: impl AsRef<Utf8Path>) -> Result<Self, LoadError> {
        let dir_path = dir_path.as_ref();
        for file_name in [
            #[cfg(feature = "gleam")]
            "gleam.toml",
            #[cfg(feature = "js")]
            "package.json",
            #[cfg(feature = "dart")]
            "pubspec.yaml",
            #[cfg(feature = "python")]
            "pyproject.toml",
            #[cfg(feature = "ruby")]
            ".gemspec.yaml", // TODO
            // This should be last, to support polyglot projects:
            #[cfg(feature = "rust")]
            "Cargo.toml",
        ] {
            let file_path = dir_path.join(file_name);
            match std::fs::symlink_metadata(&file_path) {
                Ok(_) => {},
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
                Err(error) => return Err(LoadError::at_path(file_path, error)),
            }
            match Self::load(file_path) {
                Err(LoadError::NoPackageFound(_)) => continue,
                result => return result,
            }
        }
        Err(LoadError::NoPackageFound(dir_path.into()))
    }

    /// Loads package metadata from a manifest using its enabled language adapter.
    ///
    /// Cargo workspace inheritance is resolved before converting package metadata.
    /// Filesystem access requires `std`; each format also requires its language
    /// feature, such as `rust` for Cargo or `python` for pyproject manifests.
    ///
    /// # Errors
    ///
    /// Returns [`LoadError::NoPackageFound`] with the manifest path when `[package]`
    /// or `[project]` is absent. Returns [`LoadError::UnknownPackageFormat`] for
    /// unrecognized or disabled formats, and [`LoadError::AtPath`] for unreadable,
    /// malformed, or unresolved metadata, preserving the underlying error.
    #[cfg(feature = "std")]
    pub fn load(file_path: impl AsRef<Utf8Path>) -> Result<Self, LoadError> {
        let file_path = file_path.as_ref();
        Self::load_manifest(file_path).map_err(|error| match error {
            LoadError::NoPackageFound(_) | LoadError::UnknownPackageFormat(_) => error,
            error => LoadError::at_path(file_path, error),
        })
    }

    #[cfg(feature = "std")]
    fn load_manifest(file_path: &Utf8Path) -> Result<Self, LoadError> {
        // distrib 0.0.3's JS, Dart, Ruby, and Gleam file loaders unwrap parser
        // failures. Parse their existing metadata types fallibly here instead.
        Ok(match file_path.file_name() {
            #[cfg(feature = "ruby")]
            Some(".gemspec.yaml") => {
                let manifest: distrib::ruby::Gemspec =
                    Yaml::from_str(&Self::read_manifest(file_path)?)
                        .map_err(|error| LoadError::Other(error.into()))?;
                manifest.try_into()?
            },

            #[cfg(feature = "rust")]
            Some("Cargo.toml") => {
                let manifest = distrib::rust::load_cargo_toml(file_path)?;
                if manifest.package.is_none() {
                    return Err(LoadError::NoPackageFound(file_path.into()));
                }
                manifest.try_into()?
            },

            #[cfg(feature = "gleam")]
            Some("gleam.toml") => {
                let manifest: distrib::gleam::PackageConfig =
                    toml::from_str(&Self::read_manifest(file_path)?)
                        .map_err(|error| LoadError::Other(error.into()))?;
                manifest.try_into()?
            },

            #[cfg(feature = "js")]
            Some("package.json") => {
                let manifest = distrib::js::PackageJson::try_from(Self::read_manifest(file_path)?)
                    .map_err(|error| LoadError::Other(error.into()))?;
                manifest.try_into()?
            },

            #[cfg(feature = "dart")]
            Some("pubspec.yaml") => {
                let manifest: distrib::dart::Pubspec =
                    Yaml::from_str(&Self::read_manifest(file_path)?)
                        .map_err(|error| LoadError::Other(error.into()))?;
                manifest.try_into()?
            },

            #[cfg(feature = "python")]
            Some("pyproject.toml") => {
                let manifest = distrib::python::load_pyproject_toml(file_path)?;
                if manifest.project.is_none() {
                    return Err(LoadError::NoPackageFound(file_path.into()));
                }
                manifest.try_into()?
            },

            _ => {
                return Err(LoadError::UnknownPackageFormat(file_path.into()));
            },
        })
    }

    #[cfg(all(
        feature = "std",
        any(feature = "js", feature = "dart", feature = "ruby", feature = "gleam")
    ))]
    fn read_manifest(file_path: &Utf8Path) -> Result<String, LoadError> {
        std::fs::read_to_string(file_path).map_err(|error| LoadError::Other(error.into()))
    }

    pub fn to_json(&self) -> serde_json::Value {
        self.clone().into_json()
    }

    pub fn into_json(self) -> serde_json::Value {
        // Make sure to keep this in sync with `package.csv`!
        serde_json::json!({
            "language": self.language,
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

#[cfg(feature = "gleam")]
include!("package/gleam.rs");

#[cfg(feature = "js")]
include!("package/js.rs");

#[cfg(feature = "python")]
include!("package/python.rs");

#[cfg(feature = "ruby")]
include!("package/ruby.rs");

#[cfg(feature = "rust")]
include!("package/rust.rs");
