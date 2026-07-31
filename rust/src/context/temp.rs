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

    pub fn merge(&mut self, input: impl Into<Value>) {
        let input = input.into();
        let Some(input) = input.as_object() else {
            return;
        };
        for (key, val) in input {
            if !self.has_defined(key) {
                self.define(key, val.clone());
            } else if let Some(old) = self.0.get_mut(key).and_then(|v| v.as_object_mut())
                && let Some(new) = val.as_object()
            {
                for (key, val) in new {
                    old.insert(key.clone(), val.clone());
                }
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
