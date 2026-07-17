$ readmer render --help
Render a template file to standard output

Usage: readmer render [OPTIONS] [INPUTS]...

Arguments:
  [INPUTS]...  The template files to render [default: $WORKSPACE/.config/readmer/.../README.md.liquid]

Options:
      --color <COLOR>          Set the color output mode [default: auto] [possible values: auto, always, never]
  -W, --workspace <WORKSPACE>  The workspace directory to use [default: $WORKSPACE]
  -d, --debug                  Enable debugging output
  -e, --engine <ENGINE>        The templating engine to use [default: auto]
  -D, --define <DEFINES>       Define a variable and value to pass to the templating engine
  -v, --verbose...             Enable verbose output (may be repeated for more verbosity)
  -h, --help                   Print help
