// This is free and unencumbered software released into the public domain.

#![cfg(all(feature = "std", any(feature = "rust", feature = "python")))]

mod support;

use readmer::model::{LoadError, Package};
use support::ProjectDir;

#[cfg(feature = "rust")]
#[test]
fn cargo_virtual_workspace_has_no_package() {
    let project = ProjectDir::new();
    let path = project.write("Cargo.toml", "[workspace]\nmembers = []\n");

    let error = Package::load(&path).unwrap_err();
    assert!(matches!(error, LoadError::NoPackageFound(ref found) if found == &path));
    let dir = camino::Utf8Path::from_path(project.path()).unwrap();
    assert!(matches!(Package::locate(dir), Err(LoadError::NoPackageFound(found)) if found == dir));

    let manifest = distrib::rust::load_cargo_toml(&path).unwrap();
    let error = Package::try_from(manifest).unwrap_err();
    assert!(error.to_string().contains("[package]"));
}

#[cfg(feature = "python")]
#[test]
fn tool_only_pyproject_has_no_package() {
    let project = ProjectDir::new();
    let path = project.write("pyproject.toml", "[tool.example]\nenabled = true\n");

    let error = Package::load(&path).unwrap_err();
    assert!(matches!(error, LoadError::NoPackageFound(ref found) if found == &path));
    let dir = camino::Utf8Path::from_path(project.path()).unwrap();
    assert!(matches!(Package::locate(dir), Err(LoadError::NoPackageFound(found)) if found == dir));

    let manifest = distrib::python::load_pyproject_toml(&path).unwrap();
    let error = Package::try_from(manifest).unwrap_err();
    assert!(error.to_string().contains("[project]"));
}

#[cfg(feature = "rust")]
#[test]
fn unresolved_cargo_metadata_returns_errors() {
    for field in [
        "edition",
        "rust-version",
        "version",
        "authors",
        "description",
        "homepage",
        "keywords",
        "categories",
        "license",
        "repository",
    ] {
        let manifest = distrib::rust::Manifest::from_str(&format!(
            "[package]\nname = 'example'\n{field}.workspace = true\n"
        ))
        .unwrap();
        assert!(Package::try_from(manifest).is_err(), "inherited {field}");
    }
}

#[cfg(feature = "rust")]
#[test]
fn cargo_workspace_inheritance_still_loads() {
    let project = ProjectDir::new();
    project.write(
        "Cargo.toml",
        "[workspace]\nmembers = ['member']\n\
         [workspace.package]\nversion = '1.2.3'\nedition = '2024'\nrust-version = '1.88'\n\
         authors = ['Example Author']\ndescription = 'Example description'\n\
         homepage = 'https://example.com'\nkeywords = ['example']\ncategories = ['text-processing']\n\
         license = 'MIT'\nrepository = 'https://example.com/repo'\n",
    );
    let path = project.write(
        "member/Cargo.toml",
        "[package]\nname = 'example'\nversion.workspace = true\nedition.workspace = true\n\
         rust-version.workspace = true\nauthors.workspace = true\ndescription.workspace = true\n\
         homepage.workspace = true\nkeywords.workspace = true\ncategories.workspace = true\n\
         license.workspace = true\nrepository.workspace = true\n",
    );
    project.write("member/src/lib.rs", "");

    let package = Package::load(&path).unwrap();
    assert_eq!(package.name, "example");
    assert_eq!(package.version, "1.2.3");
    assert_eq!(package.language.minimum_version.as_deref(), Some("1.88"));
    assert_eq!(package.authors, ["Example Author"]);
    assert_eq!(package.description.as_deref(), Some("Example description"));
    assert_eq!(package.homepage.as_deref(), Some("https://example.com"));
    assert_eq!(package.keywords, ["example"]);
    assert_eq!(package.categories, ["text-processing"]);
    assert_eq!(package.licenses, ["MIT"]);
    assert_eq!(
        package.repository.as_deref(),
        Some("https://example.com/repo")
    );
}

#[cfg(feature = "python")]
#[test]
fn python_package_metadata_still_loads() {
    let project = ProjectDir::new();
    let path = project.write(
        "pyproject.toml",
        "[project]\nname = 'example'\nversion = '1.2.3'\nrequires-python = '>=3.11'\n",
    );
    let package = Package::load(path).unwrap();
    assert_eq!(package.name, "example");
    assert_eq!(package.version, "1.2.3");
    assert_eq!(package.language.minimum_version.as_deref(), Some("3.11"));
}

#[cfg(all(feature = "rust", feature = "python"))]
#[test]
fn package_detection_skips_tool_only_manifests() {
    let project = ProjectDir::new();
    project.write("pyproject.toml", "[tool.example]\nenabled = true\n");
    project.write(
        "Cargo.toml",
        "[package]\nname = 'example'\nversion = '1.2.3'\n",
    );
    project.write("src/lib.rs", "");
    let path = camino::Utf8Path::from_path(project.path()).unwrap();
    let package = Package::locate(path).unwrap();
    assert_eq!(package.language.name, "rust");
    assert_eq!(package.name, "example");

    // A valid Python package retains precedence over Cargo.
    project.write("pyproject.toml", "[project]\nname = 'python-example'\n");
    let package = Package::locate(path).unwrap();
    assert_eq!(package.language.name, "python");

    // Parse failures are not mistaken for absent package metadata.
    project.write("pyproject.toml", "[project\n");
    let error = Package::locate(path).unwrap_err();
    assert!(
        matches!(&error, LoadError::AtPath { path, .. } if path.file_name() == Some("pyproject.toml"))
    );
    assert!(std::error::Error::source(&error).is_some());
}
