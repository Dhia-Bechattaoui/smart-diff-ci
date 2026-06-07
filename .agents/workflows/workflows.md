---
description: 
---

# Agent Workflows

This document defines standard workflows for AI agents assisting with this project.

## Workflow: Adding a New Language Parser
1.  Identify the target language's import syntax.
2.  Create a new module in `src/parsers/`.
3.  Implement the `DependencyExtractor` trait.
4.  Add unit tests with mock code files demonstrating valid and invalid imports.
5.  Update `PLAN.md` and `README.md` to reflect the new supported language.
