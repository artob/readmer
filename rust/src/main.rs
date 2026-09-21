// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use clientele::{
    StandardOptions,
    SysexitsError::{self, *},
    crates::camino::{Utf8Path, Utf8PathBuf},
    crates::clap::{Parser, Subcommand},
};
use readmer::{
    Context, DirContext, Engine, RenderError, Workspace,
    model::{LoadError, Package, Project},
};
use std::{
    default,
    io::{self, Write},
    path::PathBuf,
};
use thiserror::Error;
use tracing::{error, info, warn};

/// Readmer composes README.md files from Liquid or Jinja2 templates.
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
    /// Copy the project's README.md into its workspace template directory.
    #[clap(aliases = ["i", "in", "ini", "install"])]
    #[clap(
        after_help = "Creates README.md.liquid and project.yaml in $WORKSPACE/.config/readmer/<project-prefix>/.\nExisting files are preserved. A missing README.md creates an empty template."
    )]
    Init {
        /// The project directory to use, relative to $PWD [default: $PWD].
        project: Option<Utf8PathBuf>,

        /// Workspace root: $PWD or an ancestor, containing the project [default: $PWD's Git root or $PWD].
        #[clap(short = 'W', long)]
        workspace: Option<Utf8PathBuf>,
    },

    /// TODO: implement `readmer check`
    #[cfg(feature = "unstable")]
    #[clap(aliases = ["c", "ch", "che"])]
    Check {},

    /// Build ./README.md from templates in $WORKSPACE/.config/readmer/.
    #[cfg(feature = "unstable")]
    #[clap(aliases = ["b", "bu", "bui", "buidl"])]
    Build {
        /// The output files to build [default: ./README.md].
        outputs: Vec<Utf8PathBuf>,
    },

    /// Describe the selected project's metadata in JSON format.
    #[clap(aliases = ["d", "de", "des", "desc"])]
    Describe {
        /// The project directory to use, relative to $PWD [default: $PWD].
        project: Option<Utf8PathBuf>,

        /// The project property to output [default: all properties].
        property: Option<String>,

        /// Workspace root: $PWD or an ancestor, containing the project [default: $PWD's Git root or $PWD].
        #[clap(short = 'W', long)]
        workspace: Option<Utf8PathBuf>,

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

        /// Workspace root: $PWD or an ancestor, containing the project [default: $PWD's Git root or $PWD].
        #[clap(short = 'W', long)]
        workspace: Option<Utf8PathBuf>,

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
    LoadError(#[from] LoadError),

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

/// CLI entry point returning a sysexits-style process status.
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

/// Parses arguments, resolves project/workspace paths, and executes the command.
///
/// # Errors
///
/// Returns errors for invalid targets, metadata, templates, or filesystem operations.
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
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
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
            .with_max_level(match options.flags.verbose {
                0 => tracing::Level::ERROR,
                1 => tracing::Level::WARN,
                2 => tracing::Level::INFO,
                3 => tracing::Level::DEBUG,
                _ => tracing::Level::TRACE,
            })
            .init();
    }

    let mut result = Ok(());

    match options.command.unwrap_or_default() {
        Command::Init { project, workspace } => {
            let workspace = resolve_workspace(project, workspace)?;
            init_project(&workspace)?;
        },

        #[cfg(feature = "unstable")]
        Command::Check {} => {
            // TODO: implement `readmer check`
        },

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
            workspace,
            output,
            defines,
        } => {
            let workspace = resolve_workspace(project, workspace)?;
            let mut context = DirContext { workspace }.load()?;
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
            let workspace = resolve_workspace(None, workspace)?;
            let mut context = DirContext {
                workspace: workspace.clone(),
            }
            .load()?;
            for define in defines {
                let (k, v) = define
                    .split_once('=')
                    .ok_or_else(|| InvalidDefineFormat(define.clone()))?;
                context.define(k, v);
            }

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
                        // prefix is the project's path within the workspace:
                        let input_name = workspace.project_prefix().join(input_path);
                        let input_path = workspace.config_path().join(&input_name);
                        (input_name.into_string(), input_path)
                    }
                })
                .collect();

            for (template_name, template_path) in inputs {
                let engine_name = engine
                    .as_deref()
                    .or_else(|| template_path.extension())
                    .unwrap_or("liquid");
                let mut engine: Box<dyn Engine> = match engine_name {
                    "liquid" => Box::new(readmer::LiquidEngine::new(workspace.clone())),
                    "minijinja" | "jinja" | "jinja2" | "j2" => {
                        Box::new(readmer::MinijinjaEngine::new(workspace.clone()))
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
                let output = engine.render(template_name, Box::new(context.clone()))?;

                print!("{}", output);
            }
        },
    };

    result
}

// Resolve both arguments from the invocation directory before any command I/O.
fn resolve_workspace(
    project: Option<Utf8PathBuf>,
    root: Option<Utf8PathBuf>,
) -> io::Result<Workspace> {
    let project = project.unwrap_or_else(|| ".".into());
    match root {
        Some(root) => Workspace::new(root, project),
        None => Workspace::locate_from(project),
    }
}

// Nested projects share the workspace configuration tree, with project.yaml
// alongside their own template (and therefore exposed as subproject metadata).
fn init_project(workspace: &Workspace) -> io::Result<()> {
    let directory = workspace.project_config_path();
    let new_directory = !directory.try_exists().map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("cannot inspect `{directory}`: {error}"),
        )
    })?;
    std::fs::create_dir_all(&directory).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("cannot create directory `{directory}`: {error}"),
        )
    })?;
    if new_directory {
        warn!("Created the directory `{directory}`.");
    }

    let template = directory.join("README.md.liquid");
    if !existing_file(&template)? {
        let readme = workspace.project_path().join("README.md");
        let contents = if existing_file(&readme)? {
            std::fs::read(&readme).map_err(|error| {
                io::Error::new(error.kind(), format!("cannot read `{readme}`: {error}"))
            })?
        } else {
            Vec::new()
        };
        create_file(&template, &contents)?;
    }
    create_file(
        &directory.join("project.yaml"),
        b"# See: https://github.com/artob/readmer#template-variables\n---\n",
    )
}

fn existing_file(path: &Utf8Path) -> io::Result<bool> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_dir() => Err(io::Error::new(
            io::ErrorKind::IsADirectory,
            format!("expected a file, found a directory: `{path}`"),
        )),
        Ok(_) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(io::Error::new(
            error.kind(),
            format!("cannot inspect `{path}`: {error}"),
        )),
    }
}

fn create_file(path: &Utf8Path, contents: &[u8]) -> io::Result<()> {
    if existing_file(path)? {
        return Ok(());
    }
    info!("Creating the file `{path}`...");
    match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
    {
        Ok(mut file) => {
            file.write_all(contents).map_err(|error| {
                io::Error::new(error.kind(), format!("cannot write `{path}`: {error}"))
            })?;
            warn!("Created the file `{path}`.");
            Ok(())
        },
        // Preserve files created by another invocation after the existence check.
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(()),
        Err(error) => Err(io::Error::new(
            error.kind(),
            format!("cannot create `{path}`: {error}"),
        )),
    }
}
