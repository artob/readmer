# Readmer

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/readmer)](https://crates.io/crates/readmer)
[![Documentation](https://img.shields.io/docsrs/readmer?label=docs.rs)](https://docs.rs/readmer)

**Readmer composes `README.md` files from Jinja2 or Liquid templates.**

<sub>

[[Features](#-features)] |
[[Prerequisites](#%EF%B8%8F-prerequisites)] |
[[Installation](#%EF%B8%8F-installation)] |
[[Examples](#-examples)] |
[[Reference](#-reference)] |
[[Development](#%E2%80%8D-development)]

</sub>

<br/>

## ✨ Features

- Available both as the command-line tool [`readmer`] and a Rust library.
- Build `README.md` from Jinja2/Liquid/etc templates in `.config/readmer/`.
- Embeds `{% render 'table.csv' %}` partials as Markdown tables.
- Embeds `{% render 'data.json' %}` as pretty-printed JSON code blocks.
- Embeds `{% render 'example.rs' %}` as syntax-highlighted code blocks.
- Supports syntax highlighting for all file types recognized by [Linguist].
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- Polyglot software also available for Dart, Python, Ruby, and TypeScript.
- Cuts red tape: 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust] 1.87+ (2024 edition)

## ⬇️ Installation

### Installation of the CLI via Cargo

```bash
cargo install readmer --features=cli
```

### Installation of the Library via Cargo

```bash
cargo add --dev readmer
```

### Installation of the Library in `Cargo.toml`

Enable all default features:

```toml
[dev-dependencies]
readmer = { version = "0" }
```

Enable only specific features:

```toml
[dev-dependencies]
readmer = { version = "0", default-features = false, features = ["alloc"] }
```

## 👉 Examples

### Building the README.md File

```bash
readmer render > README.md
```

### Importing the Library

```rust
use readmer::*;
```

## 📚 Reference

[docs.rs/readmer](https://docs.rs/readmer)

### Command-Line Interface

```shellsession
$ readmer
Readmer composes README.md files from Jinja2 or Liquid templates

Usage: readmer [OPTIONS] [COMMAND]

Commands:
  describe  Describe the current project's metadata in JSON format
  render    Render a template file to standard output
  help      Print this message or the help of the given subcommand(s)

Options:
      --color <COLOR>  Set the color output mode [default: auto] [possible values: auto, always, never]
  -d, --debug          Enable debugging output
      --license        Show license information
  -v, --verbose...     Enable verbose output (may be repeated for more verbosity)
  -V, --version        Print version information
  -h, --help           Print help (see more with '--help')
```

#### `readmer describe`

```shellsession
$ readmer describe --help
Describe the current project's metadata in JSON format

Usage: readmer describe [OPTIONS] [PROJECT] [PROPERTY]

Arguments:
  [PROJECT]   The project directory to use [default: $PWD]
  [PROPERTY]  The project property to output [default: all properties]

Options:
      --color <COLOR>          Set the color output mode [default: auto] [possible values: auto, always, never]
  -W, --workspace <WORKSPACE>  The workspace directory to use [default: $WORKSPACE]
  -d, --debug                  Enable debugging output
  -o, --output <OUTPUT>        The output format to use [default: json]
  -D, --define <DEFINES>       Define a variable and value to pass to the templating engine
  -v, --verbose...             Enable verbose output (may be repeated for more verbosity)
  -h, --help                   Print help
```

#### `readmer render`

```shellsession
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
```

## 👨‍💻 Development

```bash
git clone https://github.com/artob/readmer.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https%3A%2F%2Fgithub.com%2Fartob%2Freadmer&text=Readmer)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https%3A%2F%2Fgithub.com%2Fartob%2Freadmer&title=Readmer)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https%3A%2F%2Fgithub.com%2Fartob%2Freadmer&t=Readmer)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fgithub.com%2Fartob%2Freadmer)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fgithub.com%2Fartob%2Freadmer)

[feature flags]: https://docs.rs/crate/readmer/latest/features
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[Linguist]: https://github.com/github-linguist/linguist
[Rust]: https://rust-lang.org
[`readmer`]: https://github.com/artob/readmer/tree/master/rust#command-line-interface
