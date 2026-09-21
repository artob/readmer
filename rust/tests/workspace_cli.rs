// This is free and unencumbered software released into the public domain.

#![cfg(all(feature = "std", feature = "cli"))]

#[path = "support/cli.rs"]
mod cli;
mod support;

use cli::{isolated_command, readmer};
use std::process::Command;
use support::ProjectDir;

fn assert_project_without_package(project: &ProjectDir) {
    project.write(
        ".config/readmer/project.yaml",
        "title: Standalone project\n",
    );
    let output = readmer(project).arg("describe").output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let context: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(context["project"]["title"], "Standalone project");
    assert!(context.get("package").is_none());
    assert!(context.get("git").is_none());

    #[cfg(feature = "liquid")]
    {
        project.write(
            ".config/readmer/README.md.liquid",
            "# {{ project.title }}\n",
        );
        let output = readmer(project).arg("render").output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            String::from_utf8(output.stdout).unwrap(),
            "# Standalone project\n"
        );
    }
}

#[test]
fn standalone_directory_supports_describe_and_render() {
    assert_project_without_package(&ProjectDir::new());
}

#[cfg(feature = "rust")]
#[test]
fn cargo_virtual_workspace_supports_describe_and_render() {
    let project = ProjectDir::new();
    project.write("Cargo.toml", "[workspace]\nmembers = []\n");
    assert_project_without_package(&project);
}

#[cfg(feature = "python")]
#[test]
fn tool_only_pyproject_supports_describe_and_render() {
    let project = ProjectDir::new();
    project.write("pyproject.toml", "[tool.example]\nenabled = true\n");
    assert_project_without_package(&project);
}

#[test]
fn missing_git_uses_current_directory() {
    let project = ProjectDir::new();
    project.write(".config/readmer/project.yaml", "title: Without Git\n");
    let output = readmer(&project)
        .env("PATH", project.path())
        .arg("describe")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let context: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(context["project"]["title"], "Without Git");
}

#[cfg(feature = "liquid")]
#[test]
fn nested_git_project_keeps_workspace_template_resolution() {
    let project = ProjectDir::new();
    let output = isolated_command("git", &project)
        .args(["-c", "init.templateDir=", "init", "--quiet"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    project.write(".config/readmer/project.yaml", "title: Root project\n");
    project.write(
        ".config/readmer/nested/project.yaml",
        "title: Child project\n",
    );
    project.write(
        ".config/readmer/nested/README.md.liquid",
        "{{ project.title }} / {{ subproject.title }}\n",
    );
    project.write("nested/.gitkeep", "");
    let output = readmer(&project)
        .current_dir(project.path().join("nested"))
        .arg("render")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "Root project / Child project\n"
    );
}

#[cfg(unix)]
#[test]
fn git_permission_errors_are_reported_without_panicking() {
    let project = ProjectDir::new();
    project.write("bin/git", "not executable\n");
    let output = readmer(&project)
        .env("PATH", project.path().join("bin"))
        .arg("describe")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!stderr.contains("panicked"), "{stderr}");
    assert!(stderr.contains("error"), "{stderr}");
}

fn assert_loading_error(mut command: Command, arguments: &[&str], path: &str) -> String {
    let output = command.args(arguments).output().unwrap();
    assert!(!output.status.success(), "unexpected success: {output:?}");
    assert!(output.stdout.is_empty(), "{output:?}");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!stderr.contains("panicked"), "{stderr}");
    assert!(stderr.replace('\\', "/").contains(path), "{stderr}");
    stderr
}

#[test]
fn malformed_project_yaml_is_reported() {
    for contents in ["title: [", "links: not-an-array\n"] {
        let project = ProjectDir::new();
        project.write(".config/readmer/project.yaml", contents);
        assert_loading_error(readmer(&project), &["describe"], "project.yaml");
        #[cfg(feature = "liquid")]
        {
            project.write(".config/readmer/README.md.liquid", "Valid template\n");
            assert_loading_error(readmer(&project), &["render"], "project.yaml");
        }
    }
}

#[test]
fn unreadable_project_yaml_is_reported() {
    let project = ProjectDir::new();
    project.write(".config/readmer/project.yaml/child", "");
    assert_loading_error(readmer(&project), &["describe"], "project.yaml");
}

#[test]
fn invalid_utf8_project_yaml_is_reported() {
    let project = ProjectDir::new();
    let path = project.write(".config/readmer/project.yaml", "");
    std::fs::write(path, [0xff]).unwrap();
    let error = assert_loading_error(readmer(&project), &["describe"], "project.yaml");
    assert!(error.to_lowercase().contains("utf-8"), "{error}");
}

#[test]
fn malformed_subproject_yaml_is_reported() {
    let project = ProjectDir::new();
    project.write(".config/readmer/project.yaml", "title: Root\n");
    project.write(".config/readmer/nested/project.yaml", "title: [");
    project.write("nested/.gitkeep", "");
    let mut command = readmer(&project);
    command.current_dir(project.path().join("nested"));
    assert_loading_error(
        command,
        &["describe", "--workspace", ".."],
        "nested/project.yaml",
    );
}

#[test]
fn missing_yaml_still_allows_environment_metadata() {
    let project = ProjectDir::new();
    let output = readmer(&project)
        .env("READMER_TITLE", "Environment title")
        .arg("describe")
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let context: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(context["project"]["title"], "Environment title");
    assert!(context.get("package").is_none());

    let mut command = readmer(&project);
    command.env("READMER_LINKS", "not-an-array");
    let error = assert_loading_error(command, &["describe"], "project.yaml");
    assert!(error.to_lowercase().contains("links"), "{error}");
}

#[test]
fn missing_yaml_does_not_search_ancestor_directories() {
    let project = ProjectDir::new();
    project.write(".config/readmer/project.yaml", "title: Ancestor\n");
    project.write("nested/.gitkeep", "");
    let output = readmer(&project)
        .current_dir(project.path().join("nested"))
        .arg("describe")
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let context: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(context["project"]["title"].is_null(), "{context}");
}

#[test]
fn malformed_package_manifests_are_reported() {
    let manifests: &[(&str, &str)] = &[
        #[cfg(feature = "rust")]
        ("Cargo.toml", "[package"),
        #[cfg(feature = "python")]
        ("pyproject.toml", "[project"),
        #[cfg(feature = "js")]
        ("package.json", "{"),
        #[cfg(feature = "dart")]
        ("pubspec.yaml", "name: ["),
        #[cfg(feature = "ruby")]
        (".gemspec.yaml", "["),
        #[cfg(feature = "gleam")]
        ("gleam.toml", "[package"),
    ];
    for (path, contents) in manifests {
        let project = ProjectDir::new();
        project.write(path, contents);
        assert_loading_error(readmer(&project), &["describe"], path);
    }
}

#[test]
fn fallible_package_parsers_preserve_valid_metadata() {
    let manifests: &[(&str, &str)] = &[
        #[cfg(feature = "js")]
        ("package.json", r#"{"name":"example","version":"1.2.3"}"#),
        #[cfg(feature = "dart")]
        ("pubspec.yaml", "name: example\nversion: 1.2.3\n"),
        #[cfg(feature = "ruby")]
        (
            ".gemspec.yaml",
            "--- !ruby/object:Gem::Specification\nname: example\nversion: !ruby/object:Gem::Version\n  version: 1.2.3\n",
        ),
        #[cfg(feature = "gleam")]
        ("gleam.toml", "name = 'example'\nversion = '1.2.3'\n"),
    ];
    for (path, contents) in manifests {
        let project = ProjectDir::new();
        project.write(path, contents);
        let output = readmer(&project).arg("describe").output().unwrap();
        assert!(output.status.success(), "{output:?}");
        let context: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(context["package"]["name"], "example", "{path}");
        assert_eq!(context["package"]["version"], "1.2.3", "{path}");
    }
}

#[cfg(all(unix, feature = "rust"))]
#[test]
fn dangling_metadata_symlinks_are_errors_not_absent_files() {
    for path in [".config/readmer/project.yaml", "Cargo.toml"] {
        let project = ProjectDir::new();
        let link = project.write(path, "");
        std::fs::remove_file(&link).unwrap();
        std::os::unix::fs::symlink("nonexistent", &link).unwrap();
        assert_loading_error(readmer(&project), &["describe"], path);
    }
}

#[cfg(feature = "liquid")]
#[test]
fn partial_loading_errors_survive_liquid_extension_fallback() {
    let project = ProjectDir::new();
    let path = project.write("broken.md", "");
    std::fs::write(path, [0xff]).unwrap();
    project.write("broken.md.liquid", "Fallback must not hide the error\n");
    project.write(
        ".config/readmer/README.md.liquid",
        "{% render 'broken.md' %}",
    );
    let error = assert_loading_error(readmer(&project), &["render"], "broken.md");
    assert!(error.to_lowercase().contains("utf-8"), "{error}");
}

#[cfg(feature = "liquid")]
#[test]
fn partial_parse_errors_survive_liquid_extension_fallback() {
    let project = ProjectDir::new();
    project.write("broken.liquid", "{% if true %}");
    project.write("broken.liquid.liquid", "Fallback must not hide the error\n");
    project.write(
        ".config/readmer/README.md.liquid",
        "{% render 'broken.liquid' %}",
    );
    let error = assert_loading_error(readmer(&project), &["render"], "broken.liquid");
    assert!(error.contains("endif"), "{error}");
}

#[cfg(feature = "liquid")]
#[test]
fn partial_fallback_only_skips_absent_files() {
    let project = ProjectDir::new();
    project.write(
        ".config/readmer/nested/README.md.liquid",
        "{% render 'snippet.md' %}",
    );
    project.write("snippet.md", "Workspace partial\n");
    project.write("nested/.gitkeep", "");
    let mut command = readmer(&project);
    command.current_dir(project.path().join("nested"));
    let output = command
        .args(["render", "--workspace", ".."])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "Workspace partial"
    );

    let path = project.write("nested/snippet.md", "");
    std::fs::write(path, [0xff]).unwrap();
    let mut command = readmer(&project);
    command.current_dir(project.path().join("nested"));
    let error = assert_loading_error(command, &["render", "--workspace", ".."], "snippet.md");
    assert!(error.to_lowercase().contains("utf-8"), "{error}");
}

#[cfg(all(feature = "liquid", feature = "csv"))]
#[test]
fn csv_partial_errors_include_path_and_cause() {
    let project = ProjectDir::new();
    project.write("broken.csv", "one,two\nonly-one\n");
    project.write(
        ".config/readmer/README.md.liquid",
        "{% render 'broken.csv' %}",
    );
    let error = assert_loading_error(readmer(&project), &["render"], "broken.csv");
    assert!(error.contains("CSV"), "{error}");

    // Header errors must propagate too, even without any data rows.
    std::fs::write(project.path().join("broken.csv"), [0xff, b'\n']).unwrap();
    let error = assert_loading_error(readmer(&project), &["render"], "broken.csv");
    assert!(error.to_lowercase().contains("utf-8"), "{error}");
}

#[test]
fn initialized_and_empty_project_yaml_remain_valid() {
    let project = ProjectDir::new();
    let output = readmer(&project).arg("init").output().unwrap();
    assert!(output.status.success(), "{output:?}");
    let output = readmer(&project).arg("describe").output().unwrap();
    assert!(output.status.success(), "{output:?}");
    let context: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(context["project"].is_object());

    project.write(".config/readmer/project.yaml", "");
    let output = readmer(&project).arg("describe").output().unwrap();
    assert!(output.status.success(), "{output:?}");
}

#[cfg(feature = "liquid")]
#[test]
fn missing_partials_and_builtin_precedence_remain_correct() {
    let project = ProjectDir::new();
    project.write(
        ".config/readmer/README.md.liquid",
        "{% render 'absent.md' %}",
    );
    assert_loading_error(readmer(&project), &["render"], "absent.md");

    project.write(
        ".config/readmer/README.md.liquid",
        "{% render 'badge/unlicense' %}",
    );
    let path = project.write("badge/unlicense.liquid", "");
    std::fs::write(path, [0xff]).unwrap();
    let output = readmer(&project).arg("render").output().unwrap();
    assert!(output.status.success(), "{output:?}");
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("https://unlicense.org")
    );

    project.write(".config/readmer/README.md.liquid", "{% render 'custom' %}");
    project.write("custom.liquid", "Custom partial\n");
    let output = readmer(&project).arg("render").output().unwrap();
    assert!(output.status.success(), "{output:?}");
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "Custom partial");
}
