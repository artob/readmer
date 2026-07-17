# Readmer: READMEs Made Simple<sup>™</sup>

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/readmer)](https://crates.io/crates/readmer)
[![Package on NPM](https://img.shields.io/npm/v/readmer.js)](https://npmjs.com/package/readmer.js)
[![Package on Pub.dev](https://img.shields.io/pub/v/readmer)](https://pub.dev/packages/readmer)
[![Package on PyPI](https://img.shields.io/pypi/v/readmer)](https://pypi.org/project/readmer)
[![Package on RubyGems](https://img.shields.io/gem/v/readmer)](https://rubygems.org/gems/readmer)

**Readmer composes `README.md` files from Liquid or Jinja2 templates.**

## ✨ Features

- Available both as the command-line tool [`readmer`] and a polyglot library.
- Build `README.md` from Liquid/Jinja2/etc templates in `.config/readmer/`.
- Embeds `{% render 'table.csv' %}` partials as Markdown tables.
- Embeds `{% render 'data.json' %}` as pretty-printed JSON code blocks.
- Embeds `{% render 'example.rs' %}` as syntax-highlighted code blocks.
- Supports syntax highlighting for all file types recognized by [Linguist].
- Polyglot software available for Dart, Python, Ruby, Rust, and TypeScript.
- Cuts red tape: 100% free and unencumbered public domain software.

## ⬇️ Installation

### Installation of the CLI

#### Installation via [Cargo]

```bash
cargo install readmer --locked --features=cli
```

### Installation of the Library

<details>
<summary>Installation for Rust from Crates.io</summary>

#### Installation from [Crates.io]

```bash
cargo add --dev readmer
```
</details>

<details>
<summary>Installation for JavaScript/TypeScript from NPM</summary>

#### Installation from [NPM]

```bash
npm install --save-dev readmer.js
bun add --dev readmer.js
pnpm add --save-dev readmer.js
yarn add --dev readmer.js
```
</details>

<details>
<summary>Installation for Dart from Pub.dev</summary>

#### Installation from [Pub.dev]

```bash
dart pub add dev:readmer
flutter pub add dev:readmer
```
</details>

<details>
<summary>Installation for Python from PyPI</summary>

#### Installation from [PyPI]

```bash
pip install -U readmer
uv add --dev readmer
poetry add --group dev readmer
pdm add -d readmer
```
</details>

<details>
<summary>Installation for Ruby from RubyGems</summary>

#### Installation from [RubyGems]

```bash
gem install readmer
bundle add readmer --group development
```
</details>

## 👉 Examples

### Building the `README.md` File

```bash
readmer render > README.md
```

## 📚 Reference

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

### Template Languages

Readmer currently supports two template languages, with Liquid being the
default as well as the more featureful implementation:

- [Liquid], using [@cobalt-org]'s [liquid] engine written in Rust
- [Jinja2], using [@mitsuhiko]'s [minijinja] engine written in Rust

### Template Syntax

#### Outputting Properties

```liquid
The title of this project is "{{ project.title }}"
```

#### Rendering Partials

```liquid
{% render 'badge/unlicense' %}
```

#### Rendering Code Blocks

```liquid
{% render 'example.rs' %}
```

#### Rendering JSON Data

```liquid
{% render 'data.json' %}
```

#### Rendering CSV Tables

```liquid
{% render 'table.csv' %}
```

### Template Properties

| Property | Type | Sample |
| -------- | ---- | ------ |
| `project.label` | string | `"Readmer"` |
| `project.title` | string | `"Readmer: READMEs Made Simple"` |
| `project.summary` | string | `"Readmer composes README.md files from templates."` |
| `subproject.label` | string | `"Readmer.py"` |
| `subproject.title` | string | `"Readmer.py: Readmer for Python"` |
| `subproject.summary` | string | `"Readmer composes README.md files from templates."` |
| `github.account.handle` | string | `"artob"` |
| `github.account.link` | string | `"https://github.com/artob"` |
| `github.repository.slug` | string | `"artob/readmer"` |
| `github.repository.link` | string | `"https://github.com/artob/readmer"` |
| `github.repository.url` | string | `"https://github.com/artob/readmer.git"` |

### Standard Partials

| Partial | Summary |
| ------- | ------- |
| `render 'badge/unlicense'` | A 'Public Domain' badge that links to unlicense.org |
| `render 'footer/share'` | A set of badges for sharing to X, Reddit, HN, Facebook, and LinkedIn |
| `render 'header/badges/dart'` | Some common above-the-fold badges for Dart projects |
| `render 'header/badges/js'` | Some common above-the-fold badges for JavaScript/TypeScript projects |
| `render 'header/badges/python'` | Some common above-the-fold badges for Python projects |
| `render 'header/badges/ruby'` | Some common above-the-fold badges for Ruby projects |
| `render 'header/badges/rust'` | Some common above-the-fold badges for Rust projects |
| `render 'header/toc'` | Quick jump links to common Table of Contents (ToC) sections |
| `render 'section/development'` | A shrink-wrap 'Development' section with Git instructions |

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
[NPM]: https://npmjs.com/package/readmer.js
[Pub.dev]: https://pub.dev/packages/readmer
[PyPI]: https://pypi.org/project/readmer
[RubyGems]: https://rubygems.org/gems/readmer

[Cargo]: https://rustup.rs
[Jinja2]: https://jinja.palletsprojects.com
[Linguist]: https://github.com/github-linguist/linguist
[Liquid]: https://shopify.dev/docs/api/liquid
[liquid]: https://crates.io/crates/liquid
[minijinja]: https://crates.io/crates/minijinja

[@cobalt-org]: https://github.com/cobalt-org
[@mitsuhiko]: https://github.com/mitsuhiko
