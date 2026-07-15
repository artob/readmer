// This is free and unencumbered software released into the public domain.

use alloc::string::String;
use serde_json::{Map, Value};

#[derive(Clone, Debug)]
pub struct Context(Map<String, Value>);

impl Context {
    pub fn new() -> Self {
        Self(Map::new())
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
