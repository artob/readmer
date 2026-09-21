$ readmer init --help
Copy the project's README.md into its workspace template directory

Usage: readmer init [OPTIONS] [PROJECT]

Arguments:
  [PROJECT]  The project directory to use, relative to $PWD [default: $PWD]

Options:
      --color <COLOR>          Set the color output mode [default: auto] [possible values: auto,
                               always, never]
  -W, --workspace <WORKSPACE>  Workspace root: $PWD or an ancestor, containing the project [default:
                               $PWD's Git root or $PWD]
  -d, --debug                  Enable debugging output
  -v, --verbose...             Enable verbose output (may be repeated for more verbosity)
  -h, --help                   Print help

Creates README.md.liquid and project.yaml in $WORKSPACE/.config/readmer/<project-prefix>/.
Existing files are preserved. A missing README.md creates an empty template.
