// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

use clientele::{
    StandardOptions,
    SysexitsError::{self, *},
    crates::clap::{Parser, Subcommand},
};
use std::path::PathBuf;

/// Readmer composes README.md files from templates.
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
    #[clap(aliases = ["b", "bu", "bui", "buidl"])]
    Build {
        /// The output files to build [default: ./README.md].
        outputs: Vec<PathBuf>,
    },

    /// Describe the current project's metadata in JSON format.
    #[clap(aliases = ["d", "de", "des", "desc"])]
    Describe {
        /// The project directory to use [default: $PWD].
        project: Option<PathBuf>,

        /// The project property to output [default: all properties].
        property: Option<String>,

        /// The output format to use.
        #[clap(short, long, default_value = "json")]
        output: Option<String>,
    },

    /// Render a template file to standard output.
    #[clap(aliases = ["r", "re", "ren"])]
    Render {
        /// The template files to render [default: $WORKSPACE/.config/readmer/.../README.md.liquid].
        inputs: Vec<PathBuf>,

        /// The templating engine to use.
        #[clap(short, long, default_value = "liquid")]
        engine: Option<String>,

        /// Define a variable and value to pass to the templating engine.
        #[clap(short = 'D', long)]
        define: Option<String>,
    },
}

impl Default for Command {
    fn default() -> Self {
        Self::Build {
            outputs: Vec::new(),
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
        Command::Build { .. } => {
            // TODO: implement `readmer build`
            Ok(())
        },
        Command::Describe { .. } => {
            // TODO: implement `readmer describe`
            Ok(())
        },
        Command::Render { .. } => {
            // TODO: implement `readmer render`
            Ok(())
        },
    }
}
