# smart-diff-ci

A blazing fast, language-agnostic Test Impact Analyzer for CI/CD pipelines.

Instead of running your entire test suite every time you push code, `smart-diff-ci` analyzes the Git diff, traverses your project's dependency graph, and strictly runs **only the tests that were impacted by your changes**.

## Installation

You can install `smart-diff-ci` globally via NPM:

```bash
npm install -g smart-diff-ci
```

## Usage

### 1. Analyze Mode (Dry Run)
Find out which tests are affected without running them:
```bash
smart-diff-ci analyze --base origin/main
```

### 2. Run Mode (Wrapper)
Pass your test runner command to `smart-diff-ci`, and it will dynamically append the affected test files and run them for you. It perfectly bubbles up the exit codes so your CI pipeline fails if a test fails!

```bash
# Flutter
smart-diff-ci run --base origin/main -- flutter test

# Node.js / Jest
smart-diff-ci run --base origin/main -- npx jest

# Python / PyTest
smart-diff-ci run --base origin/main -- pytest
```

## Configuration
You can customize exactly what constitutes a "test file" by creating a `smart-diff.yaml` file in the root of your repository:

```yaml
test_patterns:
  - ".*_test\\.dart$"
  - ".*\\.spec\\.ts$"
  - "^test_.*\\.py$"
```

## Why it's fast
`smart-diff-ci` is written purely in **Rust** and distributed as a pre-compiled binary. This NPM package is simply a lightweight wrapper that instantly downloads the native binary for your OS (Mac, Linux, or Windows). 

For more information and source code, visit the [GitHub Repository](https://github.com/dhia-bechattaoui/smart-diff-ci).
