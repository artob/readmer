// This is free and unencumbered software released into the public domain.

use alloc::string::String;
use serde_json::{Map, Value, json};

#[derive(Clone, Debug)]
pub struct Context(Map<String, Value>);

impl Context {
    pub fn new() -> Self {
        Self(Map::new())
    }

    pub fn define(&mut self, name: impl Into<String>, value: impl Into<Value>) {
        self.0.insert(name.into(), value.into());
    }

    pub fn merge_json(&mut self, json: impl Into<Value>) {
        if let Some(obj) = json.into().as_object() {
            for (key, val) in obj {
                self.define(key, val.clone());
            }
        }
    }

    pub fn to_json(&self) -> Value {
        Value::Object(self.0.clone())
    }

    pub fn into_json(self) -> Value {
        Value::Object(self.0)
    }
}

impl From<cargo_toml::Manifest> for Context {
    fn from(manifest: cargo_toml::Manifest) -> Self {
        assert!(!manifest.needs_workspace_inheritance());
        let package = manifest.package();
        let mut context = Self::new();
        context.define(
            "package",
            json!({
                "name": package.name,
                "authors": package.authors,
                "description": package.description,
                "homepage": package.homepage,
                "keywords": package.keywords,
                "categories": package.categories,
                "license": package.license,
                "repository": package.repository,
            }),
        );
        package.metadata.as_ref().map(|metadata| {
            metadata.get("readmer").map(|readmer| {
                context.merge_json(json!(readmer));
            });
        });
        context
    }
}
