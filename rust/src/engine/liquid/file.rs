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
    sync::Arc,
};
use thiserror::Error as ThisError;

pub type FilePartials = LazyCompiler<FileSource>;

/// Filesystem partials searched in order, with fallback only for absent entries.
///
/// Requires `std` and `liquid`. Use [`PartialSource::get`] for path-aware errors;
/// [`PartialSource::try_get`] intentionally discards errors for optional lookup.
#[derive(Clone, Debug, Default)]
pub struct FileSource {
    dirs: Vec<RootedPath>,
}

#[derive(Debug, ThisError)]
#[error("failed to load partial `{path}`: {source}")]
struct PartialFileError {
    path: Utf8PathBuf,
    #[source]
    source: Error,
}

impl FileSource {
    pub fn new(dirs: Vec<RootedPath>) -> Self {
        Self { dirs }
    }

    fn load(&self, name: &str) -> Result<String> {
        for dir in &self.dirs {
            let path = dir.join(name);
            let result = match std::fs::symlink_metadata(&path) {
                Ok(_) => self.load_from_path(path.clone()),
                Err(error) if error.kind() == ErrorKind::NotFound => continue,
                Err(error) => Err(error),
            };
            return result
                .map_err(|source| Error::new(source.kind(), PartialFileError { path, source }));
        }
        Err(ErrorKind::NotFound.into())
    }

    fn load_from_path(&self, path: Utf8PathBuf) -> Result<String> {
        let languages = detect_language_by_filename(&path)
            .map_err(|error| Error::new(ErrorKind::InvalidFilename, error))?;
        if let Some(language) = languages.first() {
            let code = read_to_string(path)?;
            return Ok(self.format_code(language.name, code));
        }

        match path.extension() {
            Some("liquid") | Some("md") => {
                let mut output = read_to_string(path)?;
                output.truncate(output.trim_ascii_end().len());
                Ok(output)
            },

            #[cfg(feature = "csv")]
            Some(ext @ ("csv" | "tsv" | "psv")) => {
                let mut output = String::new();
                let mut reader = csv::ReaderBuilder::new()
                    .delimiter(match ext {
                        "psv" => b'|',
                        "tsv" => b'\t',
                        _ => b',',
                    })
                    .has_headers(true)
                    .from_path(path)?;
                output.push_str("{% raw -%}");
                let headers = reader.headers()?;
                output.push('|');
                for column in headers {
                    let column = column.trim_ascii_start();
                    output.push(' ');
                    output.push_str(column);
                    output.push_str(" |");
                }
                output.push('\n');
                output.push('|');
                for column in headers {
                    let column = column.trim_ascii_start();
                    output.push(' ');
                    output.push_str(&"-".repeat(column.len()));
                    output.push_str(" |");
                }
                output.push('\n');
                for record in reader.records() {
                    let record = record?;
                    output.push('|');
                    for column in &record {
                        let column = column.trim_ascii_start();
                        output.push(' ');
                        if column.contains('|') {
                            output.push_str(&column.replace('|', "\\|"));
                        } else {
                            output.push_str(column);
                        }
                        output.push_str(" |");
                    }
                    output.push('\n');
                }
                output.push_str("{%- endraw %}");
                output.truncate(output.trim_ascii_end().len());
                Ok(output)
            },

            #[cfg(feature = "rust")]
            Some("rs") => {
                let code = read_to_string(path)?;
                Ok(self.format_code("rust", code))
            },

            Some("json") | Some(_) => {
                let languages = detect_language_by_extension(&path)
                    .map_err(|error| Error::new(ErrorKind::InvalidFilename, error))?;
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
}

impl PartialSource for FileSource {
    fn contains(&self, name: &str) -> bool {
        self.dirs
            .iter()
            .any(|dir| match std::fs::symlink_metadata(dir.join(name)) {
                Ok(_) => true,
                Err(error) => error.kind() != ErrorKind::NotFound,
            })
    }

    fn names<'a>(&'a self) -> Vec<&'a str> {
        Vec::new() // LazyCompiler doesn't call this method
    }

    fn try_get<'a>(&'a self, name: &str) -> Option<Cow<'a, str>> {
        match self.load(name) {
            Err(error) => {
                tracing::info!("Failed to read the file {name:?}: {error:?}.");
                None
            },
            Ok(content) => {
                tracing::info!("Loaded the file {name:?}.");
                Some(Cow::Owned(content))
            },
        }
    }

    fn get<'a>(&'a self, name: &str) -> liquid_core::Result<Cow<'a, str>> {
        self.load(name).map(Cow::Owned).map_err(|error| {
            liquid::Error::with_msg("Failed to load partial")
                .context("partial", name.to_string())
                .context("cause", error.to_string())
                .cause(Arc::new(error))
        })
    }
}
