# Implementation Plan: Smart-Diff CI (Test Impact Analysis CLI)

Smart-Diff CI is a lightweight, language-agnostic CLI tool designed to solve the high cost and slow duration of running complete test suites in CI/CD pipelines. It identifies precisely which files have changed, traces their dependencies recursively, matches them to tests, and triggers only the affected tests.

## Core Language Choice: Rust
We are building this tool in **Rust** due to its unmatched performance for text processing, zero-dependency binary distribution, and first-class tree-sitter integration.

## Proposed Architecture
1. **Change Detector (Git Parser):** Runs `git diff` to identify changed files.
2. **Dependency Engine (Static Parser):** Scans files and builds a DAG of file relationships via regex or tree-sitter.
3. **Transitive Resolver:** Crawls the DAG upwards to find affected test files.
4. **Runner Bridge:** Triggers the specific tests using the correct framework command.

## Version 0.0.1 Scope
- Basic CLI structure in Rust
- Support for Dart/Flutter, TypeScript/JavaScript, and Python.
- Simple YAML/JSON config parsing.
