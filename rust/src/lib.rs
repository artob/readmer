// This is free and unencumbered software released into the public domain.

//! Readmer composes `README.md` files from Liquid or Jinja2 templates.

#![no_std]
#![forbid(unsafe_code)]
#![allow(unused_imports)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

use clientele::crates::camino::{Utf8Path, Utf8PathBuf};
use dogma::{AncestorPath, FromPathError};

mod config;
pub use config::*;

mod context;
pub use context::*;

#[cfg(feature = "alloc")]
mod engine;
#[cfg(feature = "alloc")]
pub use engine::*;

mod error;
pub use error::*;

pub mod export;

pub mod model;

mod path;
pub use path::*;

mod tools;
pub use tools::*;

#[cfg(feature = "alloc")]
mod workspace;
#[cfg(feature = "alloc")]
pub use workspace::*;
