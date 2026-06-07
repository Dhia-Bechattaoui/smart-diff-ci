---
trigger: always_on
---

# Agent Rules

This document outlines the rules and constraints for AI agents operating within this repository.

1.  **Language:** The core application must be written in Rust.
2.  **Performance:** Parsing and dependency resolution must be optimized for speed. Avoid deep, unconstrained recursion.
3.  **Dependencies:** Keep external crates to a minimum unless necessary (e.g., `serde`, `clap`, `tree-sitter`).
4.  **Formatting:** Run `cargo fmt` and `cargo clippy` to ensure code quality.
