# Working on Readmer

Readmer generates Markdown from package metadata and Liquid/MiniJinja templates.
Stay within this repository; do not inspect parent directories.

## Map
- `rust/`: working library and CLI; Rust 2024, MSRV 1.88. No root Cargo workspace.
- `rust/src/main.rs`: `init`, `describe`, `render`; `check`/`build` are `unstable` stubs.
- `rust/src/{workspace.rs,path/,config.rs}`: workspace discovery, paths, configuration.
- `rust/src/context/`: template variables and merging.
- `rust/src/model/package.rs`, `rust/src/model/package/`: normalized metadata
  and language adapters using `distrib`.
- `rust/src/engine/`: rendering; `liquid/file.rs` embeds files/tables.
  `rust/data/` contains built-in partials. MiniJinja has fewer capabilities.
- `.config/readmer/`: this repository's README templates, project metadata, CLI
  help snapshots. Root `data/`: CSV reference catalogs, not built-in partials.
- `dart/`, `js/`, `python/`, `ruby/`: mostly package stubs. `ruby/` has a separate
  Cargo workspace and placeholder native extension; its core dependency comes
  from crates.io, not local `rust/`.

## Checks
Run Cargo commands in `rust/`; the CLI requires `--features cli`:

```sh
cargo fmt --all -- --check
cargo test --locked --features cli
cargo build --locked --features cli
```

- For API/docs changes: `cargo doc --locked --all-features --no-deps`.
- For CLI smoke checks, invoke the built binary from the intended project
  directory: current directory affects package metadata and template selection.
  From repository root: `rust/target/debug/readmer describe` or `render`.
- Add focused regression tests for behavior changes; integration tests belong
  in `rust/tests/`.
- Check feature combinations affected by changes. Reduced-feature builds and
  strict Clippy currently have failures; distinguish these from regressions.
- In `js/`, use Bun: `bun run build`; `bunx --no-install tsc --noEmit`.

## Contracts and editing
- Preserve `#![forbid(unsafe_code)]` and `alloc`/`std`/engine/language feature
  boundaries; gate filesystem/process operations with `std`. Follow local
  formatting and existing `camino`, `serde_json`, `thiserror` patterns.
- Template variable names and JSON shapes are public interfaces. When changing
  metadata, update the relevant root `data/properties/*.csv`/`data/variables.csv`.
  Update `data/partials.csv` when adding or changing built-in partials.
- Package detection checks Cargo last to support polyglot projects. Liquid
  partial lookup is built-ins, then current directory, then workspace root.
- Keep `init` idempotent and preserve existing files. CLI output belongs on
  stdout, diagnostics on stderr. Return errors for invalid user input.
- Prefer module/type rustdoc over README expansion. Document every public item you
  add or change, including relevant errors, feature requirements, and examples.
- README additions need clear value. Templates live in `.config/readmer/`;
  known root README/template drift needs no repair unless requested. `make`
  and `make readmes` regenerate READMEs; do not use them as validation commands.
- Regenerate derived files from their sources: CLI help via root `rake -B codegen`
  (Ruby 3.4+, freshly built `readmer` on PATH); `ruby/.gemspec.yaml` via
  `make -B -C ruby .gemspec.yaml`; release workflow via cargo-dist and
  `dist-workspace.toml`.
  Package versions differ; do not synchronize them incidentally.
