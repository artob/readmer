// This is free and unencumbered software released into the public domain.

use super::Context;
use crate::{
    Git, TempContext, Utf8Path, Workspace,
    model::{LoadError, Package},
};
use alloc::{format, string::String};
use core::result::Result;
use serde_json::{Map, Value, json};

#[derive(Clone, Debug, Default)]
pub struct DirContext {
    pub workspace: Workspace,
}

impl DirContext {
    /// Loads metadata for the workspace's selected project, independently of the
    /// process's current directory. Git and package discovery use that same project.
    ///
    /// Requires `std`. Missing project files or package metadata are optional.
    /// Git remote discovery is best-effort.
    ///
    /// # Errors
    ///
    /// Propagates errors from present but unreadable or invalid metadata instead
    /// of rendering with an incomplete context.
    #[cfg(feature = "std")]
    pub fn load(&self) -> Result<TempContext, LoadError> {
        let mut output = TempContext::new();

        let workspace_config = self.workspace.config();

        // Load `.config/readmer/project.yaml` if it exists:
        let root_project = workspace_config.project()?;
        output.define("project", root_project.into_json());

        let project_path = self.workspace.project_path();
        let git_remote_url =
            Git::default().execute_in(&project_path, ["remote", "get-url", "origin"]);
        if let Ok(ref url) = git_remote_url {
            output.define(
                "git",
                json!({ "remote": { "url": url } }),
                // TODO: git.branch
            );
        }

        let prefix = self.workspace.project_prefix();
        if !prefix.as_str().is_empty() {
            // Load `.config/readmer/.../project.yaml` if it exists:
            let subproject = workspace_config.subproject(prefix)?;
            output.define("subproject", subproject.into_json());
        }

        let package = match Package::locate(&project_path) {
            Ok(package) => Some(package),
            Err(LoadError::NoPackageFound(_)) => None,
            Err(error) => return Err(error),
        };
        if let Some(package) = package {
            if let Some(link) = package.repository.as_ref()
                && link.contains("github.com")
            {
                output.define(
                    "github",
                    json!({
                        // TODO: account
                        "repository": {
                            "link": link,
                            "url": format!("{}.git", link),
                        }
                    }),
                );
            }
            package.metadata.as_ref().map(|metadata| {
                metadata.get("readmer").map(|readmer| {
                    output.merge(json!(readmer));
                });
            });
            output.define("package", package.into_json());
        }

        if !output.has_defined("github")
            && let Ok(url) = git_remote_url
            && url.contains("github.com")
        {
            let slug = if let Some(path) = url.strip_prefix("https://github.com/") {
                // For example: "https://github.com/artob/readmer"
                // For example: "https://github.com/artob/readmer.git"
                Some(path.strip_suffix(".git").unwrap_or(path))
            } else if let Some(path) = url.strip_prefix("ssh://git@github.com/") {
                // For example: "ssh://git@github.com/artob/readmer.git"
                Some(path.strip_suffix(".git").unwrap_or(path))
            } else if let Some(path) = url.strip_prefix("git@github.com:") {
                // For example: "git@github.com:artob/readmer.git"
                Some(path.strip_suffix(".git").unwrap_or(path))
            } else {
                None // unknown URL formt
            };
            if let Some(slug) = slug {
                let link = format!("https://github.com/{}", slug);
                let url = format!("{}.git", link);
                output.define(
                    "github",
                    json!({ "repository": { "link": link, "url": url } }),
                );
            }
        }

        Ok(output)
    }

    pub fn to_json(&self) -> Value {
        self.clone().into_json()
    }

    pub fn into_json(self) -> Value {
        json!({}) // TODO
    }
}

impl Context for DirContext {
    fn to_json(&self) -> Value {
        self.to_json()
    }
}
