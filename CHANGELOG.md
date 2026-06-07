# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.5] - 2026-06-07

### Added
- Inverse Dependency Crawler module in `src/crawler.rs` utilizing the `ignore` crate to traverse the repository, parse imports, and construct the Dependency Graph while respecting `.gitignore`.

## [0.0.4] - 2026-06-07

### Added
- Language Parser module in `src/parser.rs` using regex for blazing fast extraction of import/require statements across Dart, TypeScript/JS, and Python files.
- Added `regex` and `ignore` dependencies to `Cargo.toml`.

## [0.0.3] - 2026-06-07

### Added
- Dependency Graph core module in `src/graph.rs` utilizing an adjacency list (inverse edges) to rapidly traverse and resolve transitive file dependencies.

## [0.0.2] - 2026-06-07

### Added
- Git Diff Change Detector module in `src/git.rs` to dynamically identify modified files using `std::process::Command`.
- Integrated Git Diff detection into the CLI's `analyze` command.

## [0.0.1] - 2026-06-07

### Added
- Initial project setup and folder structure.
- Basic documentation files (`README.md`, `PLAN.md`, `.agents/`).
- Basic Rust CLI boilerplate using `clap` and `anyhow` for argument parsing and error handling.

[Unreleased]: https://github.com/dhia-bechattaoui/smart-diff-ci/compare/v0.0.5...HEAD
[0.0.5]: https://github.com/dhia-bechattaoui/smart-diff-ci/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/dhia-bechattaoui/smart-diff-ci/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/dhia-bechattaoui/smart-diff-ci/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/dhia-bechattaoui/smart-diff-ci/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/dhia-bechattaoui/smart-diff-ci/releases/tag/v0.0.1