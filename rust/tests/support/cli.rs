// This is free and unencumbered software released into the public domain.

use crate::support::ProjectDir;
use std::process::Command;

pub(crate) fn isolated_command(
    program: impl AsRef<std::ffi::OsStr>,
    project: &ProjectDir,
) -> Command {
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

pub(crate) fn readmer(project: &ProjectDir) -> Command {
    isolated_command(env!("CARGO_BIN_EXE_readmer"), project)
}
