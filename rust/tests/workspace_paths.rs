// This is free and unencumbered software released into the public domain.

#![cfg(feature = "std")]

mod support;

use camino::{Utf8Path, Utf8PathBuf};
use readmer::{RootedPath, Workspace};
use std::io::ErrorKind;
use support::ProjectDir;

fn current_directory() -> Utf8PathBuf {
    Utf8PathBuf::from_path_buf(std::env::current_dir().unwrap())
        .unwrap()
        .canonicalize_utf8()
        .unwrap()
}

#[test]
fn explicit_workspace_resolves_project_paths_without_changing_current_directory() {
    let project = ProjectDir::new();
    project.write("member/.gitkeep", "");
    let caller = std::env::current_dir().unwrap();
    let canonical_root = current_directory();
    let directory = Utf8Path::from_path(project.path()).unwrap();
    let selected = directory.join("member").canonicalize_utf8().unwrap();
    let workspace = Workspace::new(&canonical_root, directory.join("member/../member")).unwrap();
    let prefix = selected.strip_prefix(&canonical_root).unwrap();
    assert_eq!(workspace.project_prefix(), prefix);
    assert_eq!(workspace.project_path(), selected);
    assert!(workspace.path().up.is_none());
    assert!(workspace.path().down.as_str().is_empty());
    assert_eq!(
        workspace.config_path(),
        canonical_root.join(".config/readmer")
    );
    assert_eq!(
        workspace.project_config_path(),
        canonical_root.join(".config/readmer").join(prefix)
    );
    assert_eq!(
        workspace.join("README.md"),
        canonical_root.join("README.md")
    );
    assert_eq!(std::env::current_dir().unwrap(), caller);
}

#[test]
fn parsing_workspace_roots_accepts_dot_and_absolute_paths() {
    let current = current_directory();
    for input in [".", current.as_str()] {
        let workspace: Workspace = input.parse().unwrap();
        let rooted: RootedPath = input.parse().unwrap();
        assert!(workspace.project_prefix().as_str().is_empty());
        assert!(rooted.up.is_none());
        assert!(rooted.down.as_str().is_empty());
        assert_eq!(rooted.join("README.md"), "README.md");
        assert_eq!(workspace.join("README.md"), current.join("README.md"));
    }
}

#[test]
fn rooted_path_preserves_ancestor_relative_representation() {
    let rooted = RootedPath {
        up: Some(dogma::AncestorPath::try_from(1usize).unwrap()),
        down: "member".into(),
    };
    assert_eq!(rooted.join("README.md"), "../README.md");
}

#[test]
fn rooted_path_rejects_descendant_roots() {
    let project = ProjectDir::new();
    let root = Utf8Path::from_path(project.path()).unwrap();
    assert_eq!(
        root.as_str().parse::<RootedPath>().unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        Workspace::new(root, root).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
}

#[test]
fn invalid_or_unrelated_directories_are_rejected() {
    let project = ProjectDir::new();
    project.write("one/.gitkeep", "");
    project.write("two/.gitkeep", "");
    project.write("file", "");
    let directory = Utf8Path::from_path(project.path()).unwrap();
    let root = current_directory();
    assert_eq!(
        Workspace::new(directory.join("one"), directory.join("two"))
            .unwrap_err()
            .kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        Workspace::new(&root, directory.join("file"))
            .unwrap_err()
            .kind(),
        ErrorKind::NotADirectory
    );
    assert_eq!(
        Workspace::new(directory.join("file"), &root)
            .unwrap_err()
            .kind(),
        ErrorKind::NotADirectory
    );
    assert_eq!(
        Workspace::new(&root, directory.join("missing"))
            .unwrap_err()
            .kind(),
        ErrorKind::NotFound
    );
    assert_eq!(
        Workspace::new("", &root).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
}

#[cfg(unix)]
#[test]
fn symlinked_projects_use_the_physical_workspace_prefix() {
    let project = ProjectDir::new();
    project.write("member/.gitkeep", "");
    let root = current_directory();
    let directory = Utf8Path::from_path(project.path()).unwrap();
    std::os::unix::fs::symlink("member", directory.join("alias")).unwrap();
    std::os::unix::fs::symlink(&root, directory.join("root-link")).unwrap();
    let workspace = Workspace::new(directory.join("root-link"), directory.join("alias")).unwrap();
    let selected = directory.join("member").canonicalize_utf8().unwrap();
    assert_eq!(
        workspace.project_prefix(),
        selected.strip_prefix(&root).unwrap()
    );
    assert_eq!(
        Workspace::new(directory.join("alias"), directory.join("member"))
            .unwrap_err()
            .kind(),
        ErrorKind::InvalidInput
    );
}

#[cfg(feature = "rust")]
#[test]
fn directory_context_loads_the_selected_package() {
    let project = ProjectDir::new();
    project.write(
        "member/Cargo.toml",
        "[package]\nname = 'selected'\nversion = '1.2.3'\n",
    );
    let directory = Utf8Path::from_path(project.path()).unwrap();
    let context = readmer::DirContext {
        workspace: Workspace::new(current_directory(), directory.join("member")).unwrap(),
    }
    .load()
    .unwrap()
    .into_json();
    assert_eq!(context["package"]["name"], "selected");
    assert_eq!(context["package"]["version"], "1.2.3");
}

#[cfg(feature = "liquid")]
#[test]
fn library_rendering_uses_selected_project_then_workspace_partials() {
    use readmer::{Engine, LiquidEngine, TempContext};
    let project = ProjectDir::new();
    let snippet = project
        .write("snippet.md", "Wrong fallback")
        .canonicalize_utf8()
        .unwrap();
    let root_file = project
        .write("root.md", "Workspace")
        .canonicalize_utf8()
        .unwrap();
    let root = current_directory();
    let snippet = snippet
        .strip_prefix(&root)
        .unwrap()
        .as_str()
        .replace('\\', "/");
    let root_file = root_file
        .strip_prefix(&root)
        .unwrap()
        .as_str()
        .replace('\\', "/");
    project.write(&format!("member/{snippet}"), "Selected project");
    let directory = Utf8Path::from_path(project.path()).unwrap();
    let mut engine = LiquidEngine::new(Workspace::new(&root, directory.join("member")).unwrap());
    engine
        .define_template(
            "example".into(),
            format!("{{% render '{snippet}' %}} / {{% render '{root_file}' %}}"),
        )
        .unwrap();
    let output = engine
        .render("example".into(), Box::new(TempContext::new()))
        .unwrap();
    assert_eq!(output, "Selected project / Workspace");
}
