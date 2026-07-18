// This is free and unencumbered software released into the public domain.

use alloc::{collections::BTreeMap, string::String, vec::Vec};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use serde_with::{
    OneOrMany,
    formats::{PreferMany, PreferOne},
    serde_as,
};

/// The package information for a gem, typically defined in a `.gemspec` file.
///
/// This is the tag `!ruby/object:Gem::Specification` in YAML.
///
/// Note that we (pedantically) distinguish between `Option<Vec<T>` and
/// `Vec<T>` so as to indicate whether the property was unset or set to empty
/// the input specification file.
///
/// See: <https://guides.rubygems.org/specification-reference/>
#[serde_as]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(default, tag = "@type")]
pub struct Specification<T = Metadata> {
    /// See: <https://guides.rubygems.org/specification-reference/#authors=>
    #[serde(alias = "author")]
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub authors: Vec<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#bindir>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindir: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#cert_chain>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_chain: Option<Vec<String>>,

    /// See: <https://guides.rubygems.org/specification-reference/#add_dependency>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<Dependency>>,

    /// See: <https://guides.rubygems.org/specification-reference/#description>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#email>
    #[serde(alias = "emails", skip_serializing_if = "Vec::is_empty")]
    #[serde_as(as = "OneOrMany<_, PreferOne>")]
    pub email: Vec<String>, // TODO: Option<Vec<String>>

    /// See: <https://guides.rubygems.org/specification-reference/#executables>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executables: Option<Vec<String>>,

    /// See: <https://guides.rubygems.org/specification-reference/#extensions>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,

    /// See: <https://guides.rubygems.org/specification-reference/#extensions_dir>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions_dir: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#extra_rdoc_files>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_rdoc_files: Option<Vec<String>>,

    /// See: <https://guides.rubygems.org/specification-reference/#files>
    pub files: Vec<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#homepage>
    pub homepage: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#licenses=>
    #[serde(alias = "license", skip_serializing_if = "Vec::is_empty")]
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub licenses: Vec<String>, // TODO: Option<Vec<String>>

    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<T>,

    /// See: <https://guides.rubygems.org/specification-reference/#name>
    pub name: String,

    /// See: <https://guides.rubygems.org/specification-reference/#platform=>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,

    /// See: <https://guides.rubygems.org/specification-reference/#post_install_message>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_install_message: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#rdoc_options>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdoc_options: Option<Vec<String>>,

    /// See: <https://guides.rubygems.org/specification-reference/#require_paths=>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_paths: Option<Vec<String>>,

    /// See: <https://guides.rubygems.org/specification-reference/#required_ruby_version>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_ruby_version: Option<Requirement>,

    /// See: <https://guides.rubygems.org/specification-reference/#required_rubygems_version>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_rubygems_version: Option<Requirement>,

    /// See: <https://guides.rubygems.org/specification-reference/#requirements>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<Vec<String>>,

    /// See: <https://guides.rubygems.org/specification-reference/#rubygems_version>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rubygems_version: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#signing_key>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_key: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#summary>
    pub summary: String,

    /// See: <https://guides.rubygems.org/specification-reference/#version>
    pub version: Version,
}

/// !ruby/object:Gem::Dependency
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Dependency {
    pub name: String,
    pub requirement: Requirement,
    pub r#type: String,
    pub prerelease: bool,
    pub version_requirements: Requirement,
}

/// `!ruby/object:Gem::Requirement`
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Requirement {
    pub requirements: Vec<(String, Version)>,
}

/// `!ruby/object:Gem::Version`
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Version {
    pub version: String,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Platform {
    #[default]
    Ruby,
    Current,
    #[serde(untagged)]
    Other(String),
}

/// See: <https://guides.rubygems.org/specification-reference/#metadata>
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Metadata {
    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bug_tracker_uri: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changelog_uri: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_uri: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_uri: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage_uri: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailing_list_uri: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_uri: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_uri: Option<String>,

    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub other: Map<String, Value>,
}
