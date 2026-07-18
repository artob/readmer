// This is free and unencumbered software released into the public domain.

use alloc::{collections::BTreeMap, string::String, vec::Vec};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{
    OneOrMany,
    formats::{PreferMany, PreferOne},
    serde_as,
};

pub type Map<K, V> = BTreeMap<K, V>;

/// See: <https://dart.dev/tools/pub/pubspec>
#[serde_as]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(default, tag = "@type")]
pub struct Pubspec {
    /// See: <https://dart.dev/tools/pub/pubspec#name>
    pub name: PackageName,

    /// See: <https://dart.dev/tools/pub/pubspec#version>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,

    /// See: <https://dart.dev/tools/pub/pubspec#description>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// See: <https://dart.dev/tools/pub/pubspec#homepage>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<Url>,

    /// See: <https://dart.dev/tools/pub/pubspec#repository>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Url>,

    /// See: <https://dart.dev/tools/pub/pubspec#issue-tracker>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_tracker: Option<Url>,

    /// See: <https://dart.dev/tools/pub/pubspec#documentation>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Url>,

    /// See: <https://dart.dev/tools/pub/pubspec#dependencies>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Map<String, Dependency>>,

    /// See: <https://dart.dev/tools/pub/pubspec#dependencies>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_dependencies: Option<Map<String, Dependency>>,

    /// See: <https://dart.dev/tools/pub/pubspec#dependencies>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_overrides: Option<Map<String, Dependency>>,

    /// See: <https://dart.dev/tools/pub/pubspec#sdk-constraints>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,

    /// See: <https://dart.dev/tools/pub/pubspec#executables>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executables: Option<Map<String, Option<String>>>,

    /// See: <https://dart.dev/tools/pub/pubspec#platforms>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Map<String, Option<()>>>,

    /// See: <https://dart.dev/tools/pub/pubspec#publish_to>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_to: Option<String>,

    /// See: <https://dart.dev/tools/pub/pubspec#funding>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<Vec<Url>>,

    /// See: <https://dart.dev/tools/pub/pubspec#false_secrets>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub false_secrets: Option<Vec<String>>,

    /// See: <https://dart.dev/tools/pub/pubspec#screenshots>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshots: Option<Vec<Screenshot>>,

    /// See: <https://dart.dev/tools/pub/pubspec#topics>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,

    /// See: <https://dart.dev/tools/pub/pubspec#ignored_advisories>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_advisories: Option<Vec<String>>,

    /// See: <https://dart.dev/tools/pub/pubspec#hooks>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Map<String, Hook>>,
}

/// See: <https://dart.dev/tools/pub/pubspec#dependencies>
pub type Dependency = VersionConstraint;

/// See: <https://dart.dev/tools/pub/pubspec#sdk-constraints>
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Environment {
    pub sdk: VersionConstraint,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flutter: Option<VersionConstraint>,
}

/// See: <https://dart.dev/tools/hooks#hook-configuration>
pub type Hook = Map<String, String>; // TODO

/// See: <https://dart.dev/tools/pub/pubspec#name>
pub type PackageName = String;

/// See: <https://dart.dev/tools/pub/pubspec#screenshots>
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Screenshot {
    pub path: String,
    pub description: String,
}

/// See: <https://dart.dev/tools/pub/pubspec#version>
pub type Version = String;

/// See: <https://dart.dev/tools/pub/dependencies#version-constraints>
pub type VersionConstraint = String;

/// See: <https://dart.dev/tools/pub/pubspec#repository>
pub type Url = String;
