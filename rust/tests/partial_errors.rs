// This is free and unencumbered software released into the public domain.

#![cfg(all(feature = "std", feature = "liquid"))]

use liquid::partials::PartialSource;
use readmer::{FileSource, RootedPath};

#[test]
fn partial_without_a_filename_does_not_panic() {
    let source = FileSource::new(vec![RootedPath::default()]);
    assert!(source.try_get(".").is_none());
}
