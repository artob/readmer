// This is free and unencumbered software released into the public domain.

use alloc::{borrow::Cow, string::ToString, vec, vec::Vec};
use liquid::{
    Template,
    partials::{EagerCompiler, LazyCompiler, PartialSource},
    reflection::ParserReflection,
};
use rust_embed::Embed;
use std::borrow::ToOwned;

pub type EmbedPartials = LazyCompiler<EmbedSource>;

#[derive(Clone, Debug, Default, Embed)]
#[folder = "data/"]
pub struct EmbedSource;

impl EmbedSource {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PartialSource for EmbedSource {
    fn contains(&self, name: &str) -> bool {
        Self::get(name).is_some()
    }

    fn names<'a>(&'a self) -> Vec<&'a str> {
        Vec::new() // LazyCompiler doesn't call this method
    }

    fn try_get<'a>(&'a self, name: &str) -> Option<Cow<'a, str>> {
        use alloc::string::String;
        use core::str::Utf8Error;

        fn convert_cow_bytes_to_str<'a>(
            input: Cow<'static, [u8]>,
        ) -> Result<Cow<'a, str>, Utf8Error> {
            Ok(match input {
                Cow::Borrowed(bytes) => {
                    Cow::Borrowed(core::str::from_utf8(bytes)?.trim_ascii_end())
                },
                Cow::Owned(_) => unreachable!(),
            })
        }

        match Self::get(name) {
            None => None, // file not found
            Some(embed) => convert_cow_bytes_to_str(embed.data).ok(),
        }
    }
}
