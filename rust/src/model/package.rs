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
            "readmer.gemspec.yaml", // FIXME
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
            #[cfg(feature = "rust")]
            Some("Cargo.toml") => export::rust::load_cargo_toml(file_path)?.try_into()?,

            #[cfg(feature = "js")]
            Some("package.json") => export::js::load_package_json(file_path)?.try_into()?,

            #[cfg(feature = "dart")]
            Some("pubspec.yaml") => export::dart::load_pubspec(file_path)?.try_into()?,

            #[cfg(feature = "python")]
            Some("pyproject.toml") => export::python::load_pyproject_toml(file_path)?.try_into()?,

            #[cfg(feature = "ruby")]
            Some("readmer.gemspec.yaml") => export::ruby::load_gemspec(file_path)?.try_into()?, // FIXME

            // TODO: Dart, JS, Ruby
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
impl TryFrom<export::dart::Pubspec> for Package {
    type Error = export::dart::LoadPubspecError;

    fn try_from(input: export::dart::Pubspec) -> Result<Self, Self::Error> {
        Ok(Self {
            name: input.name,
            version: input.version.unwrap_or_default(),
            authors: vec![], // deprecated since Dart 2.7
            description: input.description,
            homepage: input.homepage,
            keywords: input.topics.unwrap_or_default(),
            categories: vec![],
            licenses: vec![], // TODO: detect from `LICENSE` file
            repository: input.repository,
            metadata: None, // TODO
        })
    }
}

#[cfg(feature = "js")]
impl TryFrom<export::js::PackageJson> for Package {
    type Error = export::js::LoadPackageError;

    fn try_from(input: export::js::PackageJson) -> Result<Self, Self::Error> {
        use package_json_schema::{Person, PersonObject};
        Ok(Self {
            name: input.name.unwrap_or_default(),
            version: input.version.unwrap_or_default(),
            authors: input
                .author
                .into_iter()
                .map(|person| match person {
                    Person::String(name) => name,
                    Person::Object(person) => person.name,
                })
                .collect(),
            description: input.description,
            homepage: input.homepage,
            keywords: input.keywords.unwrap_or_default(),
            categories: vec![],
            licenses: input.license.into_iter().collect(),
            repository: None,
            // metadata: Some(serde_json::Value::Object(input_metadata.other)),
            ..Default::default()
        })
    }
}

#[cfg(feature = "python")]
impl TryFrom<export::python::PyprojectToml> for Package {
    type Error = export::python::LoadPyprojectError;

    fn try_from(input: export::python::PyprojectToml) -> Result<Self, Self::Error> {
        use export::python::{Contact, License};
        let project = input.project.unwrap();
        let project_urls = project.urls.unwrap_or_default();
        Ok(Self {
            name: project.name.to_string(),
            version: project.version.map(|v| v.to_string()).unwrap_or_default(),
            authors: project
                .authors
                .unwrap_or_default()
                .iter()
                .filter_map(Contact::name)
                .map(ToString::to_string)
                .collect(),
            description: project.description,
            homepage: project_urls.get("Homepage").cloned(),
            keywords: project.keywords.unwrap_or_default(),
            categories: project.classifiers.unwrap_or_default(),
            licenses: match project.license {
                Some(License::Spdx(s)) => vec![s],
                _ => vec![],
            },
            repository: project_urls.get("Repository").cloned(),
            metadata: None, // TODO
        })
    }
}

#[cfg(feature = "ruby")]
impl TryFrom<export::ruby::Specification> for Package {
    type Error = export::ruby::LoadGemspecError;

    fn try_from(input: export::ruby::Specification) -> Result<Self, Self::Error> {
        let input_metadata = input.metadata.unwrap_or_default();
        Ok(Self {
            name: input.name,
            version: input.version.version,
            authors: input.authors,
            description: input.description,
            homepage: input.homepage,
            keywords: vec![],
            categories: vec![],
            licenses: input.licenses,
            repository: input_metadata.source_code_uri,
            metadata: Some(serde_json::Value::Object(input_metadata.other)),
        })
    }
}

#[cfg(feature = "rust")]
impl TryFrom<export::rust::Manifest> for Package {
    type Error = export::rust::LoadManifestError;

    fn try_from(input: export::rust::Manifest) -> Result<Self, Self::Error> {
        use export::rust::Value;

        assert!(!input.needs_workspace_inheritance());
        let package = input.package.unwrap();
        Ok(Self {
            name: package.name,
            version: package.version.unwrap().to_string(),
            authors: package.authors.unwrap(),
            description: package.description.map(|x| x.unwrap()),
            homepage: package.homepage.map(|x| x.unwrap()),
            keywords: package.keywords.unwrap(),
            categories: package.categories.unwrap(),
            licenses: match package.license {
                None => vec![],
                Some(x) => vec![x.unwrap()],
            },
            repository: package.repository.map(|x| x.unwrap()),
            metadata: package.metadata.map(|x| x.try_into()).transpose()?,
        })
    }
}
