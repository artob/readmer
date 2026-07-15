// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use clientele::{
    StandardOptions,
    SysexitsError::{self, *},
    crates::camino::Utf8PathBuf,
    crates::clap::{Parser, Subcommand},
};
use readmer::{Engine, Workspace};
use std::{default, path::PathBuf};

/// Readmer composes README.md files from Jinja2 or Liquid templates.
#[derive(Debug, Parser)]
#[command(name = "Readmer", long_about)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Build ./README.md from templates in $WORKSPACE/.config/readmer/.
    #[cfg(feature = "unstable")]
    #[clap(aliases = ["b", "bu", "bui", "buidl"])]
    Build {
        /// The output files to build [default: ./README.md].
        outputs: Vec<Utf8PathBuf>,
    },

    /// Describe the current project's metadata in JSON format.
    #[cfg(feature = "unstable")]
    #[clap(aliases = ["d", "de", "des", "desc"])]
    Describe {
        /// The project directory to use [default: $PWD].
        project: Option<Utf8PathBuf>,

        /// The project property to output [default: all properties].
        property: Option<String>,

        /// The output format to use.
        #[clap(short, long, default_value = "json")]
        output: Option<String>,
    },

    /// Render a template file to standard output.
    #[clap(aliases = ["r", "re", "ren"])]
    Render {
        /// The template files to render [default: $WORKSPACE/.config/readmer/.../README.md.j2].
        inputs: Vec<Utf8PathBuf>,

        /// The workspace directory to use [default: $WORKSPACE].
        #[clap(short = 'W', long)]
        workspace: Option<Workspace>,

        /// The templating engine to use [default: auto].
        #[clap(short, long)]
        engine: Option<String>,

        /// Define a variable and value to pass to the templating engine.
        #[clap(short = 'D', long = "define")]
        defines: Vec<String>,
    },
}

impl Default for Command {
    fn default() -> Self {
        Self::Render {
            inputs: Vec::new(),
            workspace: None,
            engine: None,
            defines: Vec::new(),
        }
    }
}

pub fn main() -> Result<(), SysexitsError> {
    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    // Print the program version, if requested:
    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // Print the program license, if requested:
    if options.flags.license {
        print!("{}", include_str!("../UNLICENSE"));
        return Ok(());
    }

    // Configure debug output:
    if options.flags.debug {}

    match options.command.unwrap_or_default() {
        #[cfg(feature = "unstable")]
        Command::Build { outputs } => {
            let _outputs = if outputs.is_empty() {
                vec!["README.md".into()]
            } else {
                outputs
            };

            // TODO: implement `readmer build`

            Ok(())
        },

        #[cfg(feature = "unstable")]
        Command::Describe { project, .. } => {
            let _project = project.unwrap_or_else(|| ".".into());

            // TODO: implement `readmer describe`

            Ok(())
        },

        Command::Render {
            inputs,
            workspace,
            engine,
            defines,
        } => {
            let (workspace, prefix) = match workspace {
                Some(workspace) => (workspace, None),
                None => Workspace::locate().unwrap(),
            };

            let inputs = if inputs.is_empty() {
                vec![prefix.unwrap_or_default().join("README.md.liquid")] // TODO: find the prefix from the workspace
            } else {
                inputs
            };

            let mut context = serde_json::json!({});
            for define in defines {
                let (k, v) = define
                    .split_once('=')
                    .unwrap_or_else(|| panic!("invalid --define: {}", define));
                context[k] = serde_json::json!(v);
            }

            for input in inputs {
                let engine_name = engine
                    .as_deref()
                    .or_else(|| input.extension())
                    .unwrap_or("liquid");
                let mut engine: Box<dyn Engine> = match engine_name {
                    "liquid" => Box::new(readmer::LiquidEngine::new()),
                    "minijinja" | "jinja2" | "j2" => Box::new(readmer::MinijinjaEngine::new()),
                    _ => todo!(),
                };

                let template_name = input.to_string();
                let template_path =
                    if input.has_root() || input.starts_with(".") || input.starts_with("..") {
                        // Qualified paths are used as-is w/o further resolution:
                        input
                    } else {
                        // Unqualified paths are interpreted as template paths
                        // relative to the workspace's Readmer configuration
                        // directory (`$WORKSPACE/.config/readmer/`):
                        workspace.template_path(input)
                    };

                engine
                    .load_template(template_name.clone(), template_path)
                    .unwrap();
                let output = engine.render(template_name, context.clone()).unwrap();

                println!("{}", output);
            }

            Ok(())
        },
    }
}
