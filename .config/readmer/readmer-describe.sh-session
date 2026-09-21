$ readmer describe --help
Describe the selected project's metadata in JSON format

Usage: readmer describe [OPTIONS] [PROJECT] [PROPERTY]

Arguments:
  [PROJECT]   The project directory to use, relative to $PWD [default: $PWD]
  [PROPERTY]  The project property to output [default: all properties]

Options:
      --color <COLOR>          Set the color output mode [default: auto] [possible values: auto,
                               always, never]
  -W, --workspace <WORKSPACE>  Workspace root: $PWD or an ancestor, containing the project [default:
                               $PWD's Git root or $PWD]
  -d, --debug                  Enable debugging output
  -o, --output <OUTPUT>        The output format to use [default: json]
  -D, --define <DEFINES>       Define a variable and value to pass to the templating engine
  -v, --verbose...             Enable verbose output (may be repeated for more verbosity)
  -h, --help                   Print help
