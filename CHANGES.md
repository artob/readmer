# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.0.5 - 2026-07-19
### Added
- Parse Dart `pubspec.yaml` metadata
- Parse NPM `package.json` metadata
- Parse Python `pyproject.toml` metadata
- Parse Ruby `.gemspec.yaml` metadata
- Define `package.language`
- Define language-specific badges
### Changed
- Bump MSRV to 1.88 because of dependencies

## 0.0.4 - 2026-07-17
### Added
- Embed `{% render 'table.csv' %}` partials as Markdown tables
- Embed `{% render 'data.json' %}` as pretty-printed JSON code blocks
- Embed `{% render 'example.rs' %}` as syntax-highlighted code blocks
- Support syntax highlighting for all file types recognized by Linguist
### Changed
- Bump MSRV to 1.87 because of `std::io::ErrorKind::InvalidFilename`
- Resolve partials from the workspace as a fallback path

## 0.0.3 - 2026-07-16
### Added
- Define `project.{label,title,summary}`
- Define `github.repository.{link,url}`
- Embed some shrinkwrap partials for Liquid
### Changed
- Improve handling of relative paths
- Improve metadata configurability

## 0.0.2 - 2026-07-15
### Added
- `readmer describe --define`
### Changed
- Improve error handling
- Improve template path resolution
- Allow missing Cargo manifests

## 0.0.1 - 2026-07-15
### Added
- `readmer describe`
- `readmer render`

## 0.0.0 - 2026-07-13
