// This is free and unencumbered software released into the public domain.

#![cfg(all(feature = "std", feature = "liquid"))]

mod support;

use liquid::partials::{InMemorySource, PartialSource};
use readmer::{FileSource, RootedPath, StackSource};
use support::ProjectDir;

#[test]
fn partial_without_a_filename_does_not_panic() {
    let source = FileSource::new(vec![RootedPath::default()]);
    assert!(source.try_get(".").is_none());
}

#[test]
fn partial_get_preserves_io_cause_and_stops_source_fallback() {
    let project = ProjectDir::new();
    let path = project.write("broken.md", "");
    std::fs::write(project.path().join("broken.md"), [0xff]).unwrap();
    let mut sources = StackSource::new();
    sources.add(FileSource::new(vec![RootedPath::default()]));
    let mut fallback = InMemorySource::default();
    fallback.add(path.as_str(), "fallback");
    sources.add(fallback);

    let error = sources.get(path.as_str()).unwrap_err();
    assert!(error.to_string().contains(path.as_str()), "{error}");
    assert!(
        error.to_string().to_lowercase().contains("utf-8"),
        "{error}"
    );
    assert!(std::error::Error::source(&error).is_some());
    assert!(sources.try_get(path.as_str()).is_none());
}
