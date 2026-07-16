// This is free and unencumbered software released into the public domain.

use super::Context;
use crate::{
    TempContext, Utf8Path, Workspace,
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
    pub fn load(&self) -> Result<TempContext, LoadError> {
        let mut output = TempContext::new();

        let workspace_config = self.workspace.config();

        let root_project = workspace_config.project();
        if let Some(project) = root_project {
            output.define("project", project.into_json());
        }

        let prefix = &self.workspace.0.down;
        if !prefix.as_str().is_empty() {
            let cwd_project = workspace_config.subproject(prefix);
            if let Some(project) = cwd_project {
                output.define("subproject", project.into_json());
            }
        }

        //let package_path = project.unwrap_or_else(|| ".".into()); // TODO
        if let Some(package) = Package::locate(".").ok() {
            if let Some(repository) = package.repository.as_ref() {
                output.define(
                    "github",
                    json!({
                        // TODO: account
                        "repository": {
                            "link": repository,
                            "url": format!("{}.git", repository),
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
