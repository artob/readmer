// This is free and unencumbered software released into the public domain.

#![cfg(all(feature = "std", feature = "cli"))]

mod support;

use std::process::Command;
use support::ProjectDir;

fn isolated_command(program: impl AsRef<std::ffi::OsStr>, project: &ProjectDir) -> Command {
    let mut command = Command::new(program);
    command.current_dir(project.path());
    for (key, _) in std::env::vars_os() {
        if key.to_string_lossy().starts_with("GIT_")
            || key.to_string_lossy().starts_with("READMER_")
        {
            command.env_remove(key);
        }
    }
    // Keep Git discovery inside the test project, even though it lives in this repository.
    command.env("GIT_CEILING_DIRECTORIES", project.path().parent().unwrap());
    command
}

fn readmer(project: &ProjectDir) -> Command {
    isolated_command(env!("CARGO_BIN_EXE_readmer"), project)
}

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
