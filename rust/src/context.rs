// This is free and unencumbered software released into the public domain.

use serde_json::Value;

pub trait Context {
    fn to_json(&self) -> Value;
}

mod dir;
pub use dir::*;

mod temp;
pub use temp::*;
