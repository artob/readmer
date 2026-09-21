// This is free and unencumbered software released into the public domain.

#![cfg(all(feature = "std", feature = "cli"))]

#[path = "support/cli.rs"]
mod cli;
mod support;

use cli::{isolated_command, readmer};
use std::{fs, process::Command};
use support::ProjectDir;

fn successful(command: &mut Command) -> String {
    let output = command.output().unwrap();
    assert!(output.status.success(), "{output:?}");
    String::from_utf8(output.stdout).unwrap()
}

fn describe(command: &mut Command) -> serde_json::Value {
    serde_json::from_str(&successful(command)).unwrap()
}

fn init_git(project: &ProjectDir, directory: &str) {
    successful(
        isolated_command("git", project)
            .current_dir(project.path().join(directory))
            .args(["-c", "init.templateDir=", "init", "--quiet"]),
    );
}

#[cfg(feature = "rust")]
#[test]
fn describe_uses_selected_project_and_caller_relative_workspace() {
    let project = ProjectDir::new();
    project.write(
        "Cargo.toml",
        "[package]\nname = 'caller'\nversion = '0.0.0'\n",
    );
    project.write(
        "target repo/member/Cargo.toml",
        "[package]\nname = 'selected'\nversion = '1.2.3'\n",
    );
    project.write(".config/readmer/project.yaml", "title: Workspace\n");
    project.write(
        ".config/readmer/target repo/member/project.yaml",
        "title: Member\n",
    );

    let absolute_project = project.path().join("target repo/member");
    let absolute_workspace = project.path();
    for (directory, workspace) in [
        (
            std::path::Path::new("target repo/member"),
            std::path::Path::new("."),
        ),
        (absolute_project.as_path(), absolute_workspace),
    ] {
        let context = describe(
            readmer(&project)
                .arg("describe")
                .arg(directory)
                .arg("--workspace")
                .arg(workspace),
        );
        assert_eq!(context["package"]["name"], "selected");
        assert_eq!(context["project"]["title"], "Workspace");
        assert_eq!(context["subproject"]["title"], "Member");
    }
    let package =
        describe(readmer(&project).args(["describe", "target repo/member", "package", "-W", "."]));
    assert_eq!(package["name"], "selected");
}

#[test]
fn workspace_discovery_uses_cwd_and_git_metadata_uses_selected_project() {
    let project = ProjectDir::new();
    project.write(".config/readmer/project.yaml", "title: Workspace\n");
    project.write("caller/.gitkeep", "");
    project.write("target repo/member/.gitkeep", "");
    project.write(
        ".config/readmer/target repo/member/project.yaml",
        "title: Member\n",
    );
    project.write(
        "target repo/.config/readmer/project.yaml",
        "title: Nested Git root\n",
    );
    init_git(&project, ".");
    init_git(&project, "target repo");
    successful(isolated_command("git", &project).args([
        "remote",
        "add",
        "origin",
        "https://example.com/workspace.git",
    ]));
    successful(
        isolated_command("git", &project)
            .current_dir(project.path().join("target repo"))
            .args([
                "remote",
                "add",
                "origin",
                "https://example.com/selected.git",
            ]),
    );
    let context = describe(
        readmer(&project)
            .current_dir(project.path().join("caller"))
            .args(["describe", "../target repo/member"]),
    );
    assert_eq!(context["project"]["title"], "Workspace");
    assert_eq!(context["subproject"]["title"], "Member");
    assert_eq!(
        context["git"]["remote"]["url"],
        "https://example.com/selected.git"
    );
    let explicit = describe(
        readmer(&project)
            .current_dir(project.path().join("caller"))
            .args(["describe", "../target repo/member", "-W", ".."]),
    );
    assert_eq!(context, explicit);
}

#[test]
fn workspace_accepts_dot_absolute_and_normalized_directory_paths() {
    let project = ProjectDir::new();
    project.write(".config/readmer/project.yaml", "title: Root\n");
    project.write("nested/.gitkeep", "");
    for workspace in [
        std::path::Path::new("."),
        std::path::Path::new("nested/.."),
        project.path(),
    ] {
        let context = describe(readmer(&project).arg("describe").arg("-W").arg(workspace));
        assert_eq!(context["project"]["title"], "Root");
        assert!(context.get("subproject").is_none());
    }
}

#[test]
fn explicit_workspace_must_be_cwd_or_an_ancestor_even_with_project_selection() {
    let project = ProjectDir::new();
    project.write("nested/member/README.md", "# Member\n");
    project.write("caller/.gitkeep", "");
    for command in ["describe", "init"] {
        for workspace in [
            project.path().join("nested"),
            std::path::PathBuf::from("nested"),
        ] {
            let output = readmer(&project)
                .args([command, "nested/member", "--workspace"])
                .arg(workspace)
                .output()
                .unwrap();
            assert!(!output.status.success(), "{output:?}");
            assert!(output.stdout.is_empty(), "{output:?}");
            assert!(
                String::from_utf8_lossy(&output.stderr)
                    .contains("current directory or an ancestor"),
                "{output:?}"
            );
        }
        let output = readmer(&project)
            .current_dir(project.path().join("caller"))
            .args([command, "../nested/member", "-W", "../nested"])
            .output()
            .unwrap();
        assert!(!output.status.success(), "{output:?}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("current directory or an ancestor"),
            "{output:?}"
        );
        let output = readmer(&project)
            .current_dir(project.path().join("caller"))
            .args([command, "../nested/member", "-W", "."])
            .output()
            .unwrap();
        assert!(!output.status.success(), "{output:?}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("outside workspace"),
            "{output:?}"
        );
    }
    assert!(!project.path().join("nested/.config").exists());
}

#[test]
fn project_selection_does_not_change_non_git_workspace_discovery() {
    let project = ProjectDir::new();
    project.write(
        ".config/readmer/project.yaml",
        "title: Invocation workspace\n",
    );
    project.write(".config/readmer/member/project.yaml", "title: Member\n");
    project.write("member/.gitkeep", "");
    let context = describe(readmer(&project).args(["describe", "member"]));
    assert_eq!(context["project"]["title"], "Invocation workspace");
    assert_eq!(context["subproject"]["title"], "Member");
}

#[test]
fn invalid_targets_fail_before_reading_metadata_or_initializing() {
    let project = ProjectDir::new();
    project.write("file", "not a directory");
    project.write("nested/.gitkeep", "");
    for arguments in [
        vec!["describe", "missing"],
        vec!["describe", "file"],
        vec!["describe", "-W", "missing"],
        vec!["describe", "-W", "file"],
        vec!["describe", "-W", "nested"],
        vec!["init", "missing"],
        vec!["init", "file"],
        vec!["init", "-W", "nested"],
    ] {
        let output = readmer(&project).args(&arguments).output().unwrap();
        assert!(!output.status.success(), "{arguments:?}: {output:?}");
        assert!(output.stdout.is_empty(), "{output:?}");
        assert!(
            !String::from_utf8_lossy(&output.stderr).contains("panicked"),
            "{output:?}"
        );
    }
    assert!(!project.path().join(".config").exists());
    assert!(!project.path().join("nested/.config").exists());
}

#[cfg(feature = "liquid")]
#[test]
fn render_uses_workspace_prefix_with_absolute_root() {
    let project = ProjectDir::new();
    project.write(".config/readmer/project.yaml", "title: Root\n");
    project.write(".config/readmer/member/project.yaml", "title: Member\n");
    project.write(
        ".config/readmer/member/README.md.liquid",
        "{{ project.title }} / {{ subproject.title }} / {% render 'snippet.md' %}",
    );
    project.write("member/snippet.md", "Project partial\n");
    let output = successful(
        readmer(&project)
            .current_dir(project.path().join("member"))
            .arg("render")
            .arg("-W")
            .arg(project.path()),
    );
    assert_eq!(output, "Root / Member / Project partial");
}

#[test]
fn nested_init_uses_git_workspace_and_preserves_existing_files() {
    let project = ProjectDir::new();
    project.write(
        ".config/readmer/project.yaml",
        "title: Existing workspace\n",
    );
    project.write("member/README.md", "# Member\n");
    init_git(&project, ".");
    successful(
        readmer(&project)
            .current_dir(project.path().join("member"))
            .arg("init"),
    );
    let template = project
        .path()
        .join(".config/readmer/member/README.md.liquid");
    assert_eq!(fs::read_to_string(&template).unwrap(), "# Member\n");
    assert!(
        project
            .path()
            .join(".config/readmer/member/project.yaml")
            .is_file()
    );
    assert!(!project.path().join("member/.config").exists());

    project.write("member/README.md", "Changed source\n");
    project.write(
        ".config/readmer/member/project.yaml",
        "title: Existing member\n",
    );
    successful(
        readmer(&project)
            .current_dir(project.path().join("member"))
            .arg("init"),
    );
    assert_eq!(fs::read_to_string(template).unwrap(), "# Member\n");
    assert_eq!(
        fs::read_to_string(project.path().join(".config/readmer/member/project.yaml")).unwrap(),
        "title: Existing member\n"
    );
    assert_eq!(
        fs::read_to_string(project.path().join(".config/readmer/project.yaml")).unwrap(),
        "title: Existing workspace\n"
    );
}

#[test]
fn init_can_target_an_explicit_project_and_workspace() {
    let project = ProjectDir::new();
    project.write("target repo/member/README.md", "# Selected\n");
    successful(readmer(&project).args(["init", "target repo/member", "--workspace", "."]));
    assert_eq!(
        fs::read_to_string(
            project
                .path()
                .join(".config/readmer/target repo/member/README.md.liquid")
        )
        .unwrap(),
        "# Selected\n"
    );
    assert!(
        project
            .path()
            .join(".config/readmer/target repo/member/project.yaml")
            .is_file()
    );
    assert!(!project.path().join("target repo/.config").exists());
    assert!(!project.path().join("target repo/member/.config").exists());
}

#[cfg(unix)]
#[test]
fn init_preserves_dangling_destination_symlinks() {
    let project = ProjectDir::new();
    project.write("README.md", "# Source\n");
    for name in ["README.md.liquid", "project.yaml"] {
        let path = project.write(&format!(".config/readmer/{name}"), "");
        fs::remove_file(&path).unwrap();
        std::os::unix::fs::symlink("absent", &path).unwrap();
    }
    successful(readmer(&project).args(["init", "-W", "."]));
    for name in ["README.md.liquid", "project.yaml"] {
        assert_eq!(
            fs::read_link(project.path().join(".config/readmer").join(name)).unwrap(),
            std::path::Path::new("absent")
        );
    }
}

#[test]
fn init_does_not_create_an_empty_template_when_readme_is_unreadable() {
    let project = ProjectDir::new();
    project.write("README.md/child", "");
    let output = readmer(&project)
        .args(["init", "-W", "."])
        .output()
        .unwrap();
    assert!(!output.status.success(), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("README.md"),
        "{output:?}"
    );
    assert!(
        !project
            .path()
            .join(".config/readmer/README.md.liquid")
            .exists()
    );
}

#[cfg(unix)]
#[test]
fn git_workspace_paths_preserve_trailing_spaces() {
    let project = ProjectDir::new();
    project.write("repo /.config/readmer/project.yaml", "title: Spaced root\n");
    project.write("repo /member/.gitkeep", "");
    init_git(&project, "repo ");
    let context = describe(
        readmer(&project)
            .current_dir(project.path().join("repo /member"))
            .arg("describe"),
    );
    assert_eq!(context["project"]["title"], "Spaced root");
}
