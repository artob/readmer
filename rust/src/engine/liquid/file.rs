// This is free and unencumbered software released into the public domain.

use crate::RootedPath;
use alloc::{
    borrow::{Cow, ToOwned},
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use camino::Utf8PathBuf;
use linguist::{detect_language_by_extension, detect_language_by_filename};
use liquid::{
    Template,
    partials::{EagerCompiler, LazyCompiler, PartialSource},
    reflection::ParserReflection,
};
use std::{
    fs::read_to_string,
    io::{Error, ErrorKind, Result},
};

pub type FilePartials = LazyCompiler<FileSource>;

#[derive(Clone, Debug, Default)]
pub struct FileSource {
    base: RootedPath,
}

impl FileSource {
    pub fn new(base: RootedPath) -> Self {
        Self { base }
    }

    fn load(&self, name: &str) -> Result<String> {
        let path = self.join(name);
        if !path.try_exists()? {
            return Err(ErrorKind::NotFound.into());
        }

        let languages = detect_language_by_filename(&path).unwrap(); // FIXME
        if let Some(language) = languages.first() {
            let code = read_to_string(path)?;
            return Ok(self.format_code(language.name, code));
        }

        match path.extension() {
            Some("liquid") | Some("md") => {
                let mut result = read_to_string(path)?;
                result.truncate(result.trim_ascii_end().len());
                Ok(result)
            },
            Some("csv") | Some("tsv") => todo!(), // TODO
            Some("rs") => {
                let code = read_to_string(path)?;
                Ok(self.format_code("rust", code))
            },
            Some("json") | Some(_) => {
                let languages = detect_language_by_extension(&path).unwrap(); // FIXME
                let Some(language) = languages.first() else {
                    return Err(ErrorKind::InvalidFilename.into());
                };
                let code = read_to_string(path)?;
                let tag: String = language
                    .name
                    .chars()
                    .map(|c| match c {
                        ' ' => '-',
                        c => c.to_ascii_lowercase(),
                    })
                    .collect();
                Ok(self.format_code(tag, code))
            },
            None => return Err(ErrorKind::InvalidFilename.into()),
        }
    }

    fn format_code(&self, tag: impl AsRef<str>, code: impl Into<String>) -> String {
        let code = code.into();
        format!(
            "```{tag}\n{code}\n```",
            tag = tag.as_ref(),
            code = code.trim_ascii_end()
        )
    }

    fn join(&self, name: &str) -> Utf8PathBuf {
        self.base.join(name)
    }
}

impl PartialSource for FileSource {
    fn contains(&self, name: &str) -> bool {
        self.join(name).exists()
    }

    fn names<'a>(&'a self) -> Vec<&'a str> {
        Vec::new() // LazyCompiler doesn't call this method
    }

    fn try_get<'a>(&'a self, name: &str) -> Option<Cow<'a, str>> {
        self.load(name).ok().map(Cow::Owned)
    }
}
