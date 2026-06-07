# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-06-07

### Added
- **Test Runner Bridge (`smart-diff run`)**: Added a wrapper mode that dynamically executes the user's test runner (e.g. `flutter test`) passing only the affected files. It forwards `stdout` and bubbles up the correct exit code.
- **YAML Config Parser**: The tool now dynamically parses a `smart-diff.yaml` configuration file at the repository root to overwrite hardcoded test-matching heuristics using Regex strings.
- **Cross-Platform Automated Releases**: Automated GitHub Actions to compile and compress binaries for Windows, Linux, and macOS (Intel and ARM).
- **Official Homebrew Formula**: Built and distributed an automated Homebrew Tap for Mac users.
- **NPM Wrapper**: Shipped a native Node.js wrapper to `npmjs.com` to allow web developers to install the CLI without Rust.
- **GitHub Action Support**: Packaged the repository as a native GitHub Action.

## [0.0.8] - 2026-06-07

### Added
- Implemented **fuzzy substring matching** in the Dependency Graph core (`src/graph.rs`). This allows seamless resolution between Git's relative file paths (`lib/utils.dart`) and Language-specific package imports (e.g. Dart's `import 'package:app/utils.dart';`).

## [0.0.7] - 2026-06-07

### Added
- Git Diff module now fully supports local uncommitted changes, seamlessly analyzing staged, unstaged, and untracked files locally.
- Unified detection logic merges committed diffs against the base branch with the current working directory state.

## [0.0.6] - 2026-06-07

### Added
- Runner Bridge in `src/main.rs` to seamlessly connect the CLI commands with the Git Diff detector, Crawler, and Dependency Graph.
- Built-in heuristic to automatically isolate and filter affected test files (e.g., `_test.dart`, `.test.js`, `test_*.py`) from the complete dependency graph resolution.

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

[Unreleased]: https://github.com/dhia-bechattaoui/smart-diff-ci/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/dhia-bechattaoui/smart-diff-ci/compare/v0.0.8...v0.2.0
[0.0.8]: https://github.com/dhia-bechattaoui/smart-diff-ci/compare/v0.0.7...v0.0.8
[0.0.7]: https://github.com/dhia-bechattaoui/smart-diff-ci/compare/v0.0.6...v0.0.7
[0.0.6]: https://github.com/dhia-bechattaoui/smart-diff-ci/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/dhia-bechattaoui/smart-diff-ci/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/dhia-bechattaoui/smart-diff-ci/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/dhia-bechattaoui/smart-diff-ci/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/dhia-bechattaoui/smart-diff-ci/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/dhia-bechattaoui/smart-diff-ci/releases/tag/v0.0.1