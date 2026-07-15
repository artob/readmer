// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use clientele::{
    StandardOptions,
    SysexitsError::{self, *},
    crates::camino::Utf8PathBuf,
    crates::clap::{Parser, Subcommand},
};
use readmer::{Context, Engine, RenderError, Workspace};
use std::{default, path::PathBuf};
use thiserror::Error;
use tracing::error;

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
    #[clap(aliases = ["d", "de", "des", "desc"])]
    Describe {
        /// The project directory to use [default: $PWD].
        project: Option<Utf8PathBuf>,

        /// The project property to output [default: all properties].
        property: Option<String>,

        /// The output format to use.
        #[clap(short, long, default_value = "json")]
        output: String,

        /// Define a variable and value to pass to the templating engine.
        #[clap(short = 'D', long = "define")]
        defines: Vec<String>,
    },

    /// Render a template file to standard output.
    #[clap(aliases = ["r", "re", "ren"])]
    Render {
        /// The template files to render [default: $WORKSPACE/.config/readmer/.../README.md.liquid].
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

#[derive(Debug, Error)]
pub enum ProgramError {
    #[error("unknown --engine name: {0}")]
    UnknownEngineName(String),

    #[error("unknown --output format: {0}")]
    UnknownOutputFormat(String),

    #[error("invalid --define format: {0}")]
    InvalidDefineFormat(String),

    #[error(transparent)]
    RenderError(#[from] RenderError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Exit(#[from] SysexitsError),

    #[error(transparent)]
    Other(#[from] Box<dyn core::error::Error>),
}

impl From<ProgramError> for SysexitsError {
    fn from(error: ProgramError) -> Self {
        use ProgramError::*;
        match error {
            Exit(code) => code,
            _ => EX_SOFTWARE,
        }
    }
}

pub fn main() -> SysexitsError {
    use ProgramError::*;

    match run() {
        Ok(()) => EX_OK,
        Err(Exit(exit)) => exit,
        Err(error) => {
            // TODO: color coding
            error!("{}: error: {}", env!("CARGO_PKG_NAME"), error);
            error.into()
        },
    }
}

pub fn run() -> Result<(), ProgramError> {
    use ProgramError::*;

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
    if options.flags.debug {
        tracing_subscriber::fmt().init();
    } else {
        tracing_subscriber::fmt()
            .with_writer(std::io::stderr)
            .compact()
            .without_time()
            .with_target(false)
            .with_level(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(false)
            .with_line_number(false)
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()) // respects RUST_LOG
            .init();
    }

    let mut result = Ok(());

    match options.command.unwrap_or_default() {
        #[cfg(feature = "unstable")]
        Command::Build { outputs } => {
            let _outputs = if outputs.is_empty() {
                vec!["README.md".into()]
            } else {
                outputs
            };

            // TODO: implement `readmer build`
        },

        Command::Describe {
            project,
            property,
            output,
            defines,
        } => {
            let project_path = project.unwrap_or_else(|| ".".into());
            let manifest_path = project_path.join("Cargo.toml");

            let mut context = match cargo_toml::Manifest::from_path(&manifest_path) {
                Ok(manifest) => Context::from(manifest),
                Err(_) => Context::default(),
            };
            for define in defines {
                let (k, v) = define
                    .split_once('=')
                    .ok_or_else(|| InvalidDefineFormat(define.clone()))?;
                context.define(k, v);
            }

            match output.as_str() {
                "json" => {
                    let mut json = context.into_json();
                    if let Some(property) = property {
                        json = json.get(property).cloned().unwrap_or_default();
                    }
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&json).map_err(|e| Other(e.into()))?
                    );
                },
                _ => {
                    return Err(UnknownOutputFormat(output));
                },
            }
        },

        Command::Render {
            mut inputs,
            workspace,
            engine,
            defines,
        } => {
            let (workspace, prefix) = match workspace {
                Some(workspace) => (workspace, None),
                None => Workspace::locate().map_err(|e| Other(e.into()))?,
            };

            if inputs.is_empty() {
                // TODO: find an actual existing template, if any
                inputs.push("README.md.liquid".into());
            }
            let inputs: Vec<(String, Utf8PathBuf)> = inputs
                .into_iter()
                .map(|input_path| {
                    if input_path.has_root()
                        || input_path.starts_with(".")
                        || input_path.starts_with("..")
                        || input_path.starts_with(".config")
                    {
                        // Qualified paths are used as-is w/o further resolution,
                        // but we do try to derive a sensible template name:
                        let input_name = input_path.to_string();
                        let input_name = input_name
                            .split(".config/readmer/")
                            .last()
                            .map(ToString::to_string)
                            .unwrap_or_else(|| input_name);
                        (input_name, input_path)
                    } else {
                        // Unqualified paths are interpreted relative to the
                        // workspace's prefixed configuration directory
                        // (`$WORKSPACE/.config/readmer/$PREFIX/`), where the
                        // prefix is the relative path to the workspace root:
                        let input_name = prefix
                            .clone()
                            .unwrap_or_default()
                            .join(input_path)
                            .into_string();
                        let input_path = workspace.template_path(&input_name);
                        (input_name, input_path)
                    }
                })
                .collect();

            let mut context = match cargo_toml::Manifest::from_path("Cargo.toml") {
                Ok(manifest) => Context::from(manifest),
                Err(_) => Context::default(),
            };
            for define in defines {
                let (k, v) = define
                    .split_once('=')
                    .ok_or_else(|| InvalidDefineFormat(define.clone()))?;
                context.define(k, v);
            }

            for (template_name, template_path) in inputs {
                let engine_name = engine
                    .as_deref()
                    .or_else(|| template_path.extension())
                    .unwrap_or("liquid");
                let mut engine: Box<dyn Engine> = match engine_name {
                    "liquid" => Box::new(readmer::LiquidEngine::new()),
                    "minijinja" | "jinja" | "jinja2" | "j2" => {
                        Box::new(readmer::MinijinjaEngine::new())
                    },
                    _ => return Err(UnknownEngineName(engine_name.into())),
                };

                match engine.load_template(template_name.clone(), template_path.clone()) {
                    Ok(_) => {},
                    Err(error) => {
                        error!(
                            "{}: failed to load template `{}`: {} for `{}`",
                            env!("CARGO_PKG_NAME"),
                            &template_name,
                            error,
                            &template_path
                        );
                        result = Err(Exit(EX_DATAERR));
                        continue;
                    },
                };
                let output = engine.render(template_name, context.clone())?;

                print!("{}", output);
            }
        },
    };

    result
}
