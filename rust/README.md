# Readmer.rs

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/readmer)](https://crates.io/crates/readmer)
[![Documentation](https://img.shields.io/docsrs/readmer?label=docs.rs)](https://docs.rs/readmer)

**Readmer composes `README.md` files from templates.**

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

- Available both as the command-line tool [`readmer`] and as a Rust library.
- Compose README.md files from Liquid templates in `.config/readmer/`.
- 100% pure and safe Rust with minimal dependencies and no bloat.
- Designed for `#![no_std]` environment compatibility from the get-go.
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- Polyglot software also available for Dart, Python, Ruby, and TypeScript.
- Cuts red tape: 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition)

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add --dev readmer
```

### Installation in `Cargo.toml`

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

### Importing the Library

```rust
use readmer::*;
```

## 📚 Reference

[docs.rs/readmer](https://docs.rs/readmer)

## 👨‍💻 Development

```bash
git clone https://github.com/artob/readmer.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/artob/readmer&text=Readmer)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/artob/readmer&title=Readmer)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/artob/readmer&t=Readmer)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/artob/readmer)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/artob/readmer)

[feature flags]: https://docs.rs/crate/readmer/latest/features
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[Rust]: https://rust-lang.org
