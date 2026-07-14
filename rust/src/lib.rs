// This is free and unencumbered software released into the public domain.

//! Readmer composes `README.md` files from Jinja2 or Liquid templates.

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

#[cfg(feature = "alloc")]
mod engine;
#[cfg(feature = "alloc")]
pub use engine::*;

mod error;
pub use error::*;

mod tools;
pub use tools::*;

#[cfg(feature = "alloc")]
mod workspace;
#[cfg(feature = "alloc")]
pub use workspace::*;
