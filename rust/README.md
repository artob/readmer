# Readmer: READMEs Made Simple<sup>™</sup>

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.88%2B-blue)](https://endoflife.date/rust)
[![Package on Crates.io](https://img.shields.io/crates/v/readmer)](https://crates.io/crates/readmer)
[![Documentation](https://img.shields.io/docsrs/readmer?label=docs.rs)](https://docs.rs/readmer)

**Readmer composes `README.md` files from Liquid or Jinja2 templates.**

<sub>

[[Features](#-features)] |
[[Prerequisites](#%EF%B8%8F-prerequisites)] |
[[Installation](#%EF%B8%8F-installation)] |
[[Examples](#-examples)] |
[[Reference](#-reference)] |
[[Development](#%E2%80%8D-development)]

</sub>

<br/>

<img width="100%" alt="Showcase of readmer render" src="https://github.com/artob/readmer/raw/master/rust/etc/asciinema/demo.gif"/>

## ✨ Features

- Available both as the command-line tool [`readmer`] and a Rust library.
- Build `README.md` from Liquid/Jinja2/etc templates in `.config/readmer/`.
- Keeps you in charge: use as little or as much code generation as you like.
- Extracts package metadata for Dart, NPM, Python, Ruby, and Rust projects.
- Embeds `{% render 'table.csv' %}` partials as Markdown tables.
- Embeds `{% render 'data.json' %}` as pretty-printed JSON code blocks.
- Embeds `{% render 'example.rs' %}` as syntax-highlighted code blocks.
- Supports syntax highlighting for 800+ file types recognized by [Linguist].
- Includes numerous builtin partials for rendering header/footer badges.
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- Polyglot software also (soon!) available for Dart, Python, Ruby, and TypeScript.
- Cuts red tape: 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust] 1.88+ (2024 edition)

## ⬇️ Installation

### Installation of the CLI

#### Installation via [Cargo Binstall]

```bash
cargo binstall -y readmer
```

<img width="100%" alt="Installation via cargo-binstall" src="https://github.com/artob/readmer/raw/master/rust/etc/asciinema/install.gif"/>

#### Installation via [Cargo]

```bash
cargo install readmer --locked --features=cli
```

#### Downloading Release Tarballs

```bash
wget https://github.com/artob/readmer/releases/download/0.0.6/readmer-aarch64-apple-darwin.tar.xz
wget https://github.com/artob/readmer/releases/download/0.0.6/readmer-aarch64-unknown-linux-gnu.tar.xz
wget https://github.com/artob/readmer/releases/download/0.0.6/readmer-x86_64-apple-darwin.tar.xz
wget https://github.com/artob/readmer/releases/download/0.0.6/readmer-x86_64-pc-windows-msvc.zip
wget https://github.com/artob/readmer/releases/download/0.0.6/readmer-x86_64-unknown-linux-gnu.tar.xz
```

### Installation of the Library

<details>
<summary>Installation from Crates.io</summary>

#### Installation from [Crates.io]

```bash
cargo add --dev readmer
```
</details>

<details>
<summary>Configuration in <code>Cargo.toml</code></summary>

#### Configuration in `Cargo.toml`

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
</details>

## 👉 Examples

### Initializing the Template

Execute this in your project's workspace (aka top-level) directory, to create
an initial `.config/readmer/README.md.liquid` template:

```bash
mkdir -p .config/readmer/
echo '# {{ package.name | capitalize }}' > .config/readmer/README.md.liquid
```
### Building the `README.md` File

In your project's workspace directory, this will render the
`.config/readmer/README.md.liquid` template and write the output to `./README.md`.
In subdirectories under your workspace, it will render the corresponding
`.config/readmer/.../README.md.liquid` template if one exists.

```bash
readmer render > README.md
```

### Examining Template Variables

In any project directory, you can examine the template variables available to
you using `readmer describe`:

```bash
readmer describe
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

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https%3A%2F%2Fgithub.com%2Fartob%2Freadmer&text=Readmer%3A%20READMEs%20Made%20Simple)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https%3A%2F%2Fgithub.com%2Fartob%2Freadmer&title=Readmer%3A%20READMEs%20Made%20Simple)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https%3A%2F%2Fgithub.com%2Fartob%2Freadmer&t=Readmer%3A%20READMEs%20Made%20Simple)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fgithub.com%2Fartob%2Freadmer)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fgithub.com%2Fartob%2Freadmer)

[`readmer`]: https://github.com/artob/readmer#command-line-interface

[Crates.io]: https://crates.io/crates/readmer
[feature flags]: https://docs.rs/crate/readmer/latest/features
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[Cargo]: https://rustup.rs
[Cargo Binstall]: https://crates.io/crates/cargo-binstall
[Linguist]: https://github.com/github-linguist/linguist
[Rust]: https://rust-lang.org
