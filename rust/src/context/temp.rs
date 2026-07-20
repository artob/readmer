// This is free and unencumbered software released into the public domain.

use super::Context;
use alloc::string::String;
use serde_json::{Map, Value};

#[derive(Clone, Debug, Default)]
pub struct TempContext(Map<String, Value>);

impl TempContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn merge(&mut self, json: impl Into<Value>) {
        if let Some(obj) = json.into().as_object() {
            for (key, val) in obj {
                self.define(key, val.clone());
            }
        }
    }

    pub fn has_defined(&self, name: impl AsRef<str>) -> bool {
        self.0.contains_key(name.as_ref())
    }

    pub fn define(&mut self, name: impl Into<String>, value: impl Into<Value>) {
        self.0.insert(name.into(), value.into());
    }

    pub fn to_json(&self) -> Value {
        Value::Object(self.0.clone())
    }

    pub fn into_json(self) -> Value {
        Value::Object(self.0)
    }
}

impl Context for TempContext {
    fn to_json(&self) -> Value {
        self.to_json()
    }
}
