use anyhow::{Context, Result};
use std::collections::HashSet;
use std::process::Command;

fn run_git_diff(args: &[&str]) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute git {:?}", args))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git diff failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let files = stdout
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(files)
}

/// Retrieves a unified list of changed files, including:
/// 1. Committed changes between base_branch and HEAD
/// 2. Local uncommitted changes (staged and unstaged)
pub fn get_changed_files(base_branch: &str) -> Result<Vec<String>> {
    let mut all_files = HashSet::new();

    // 1. Committed changes on the current branch vs the base branch
    let committed_target = format!("{}...HEAD", base_branch);
    let committed_args = vec!["diff", "--name-only", &committed_target];
    if let Ok(files) = run_git_diff(&committed_args) {
        all_files.extend(files);
    }

    // 2. Local uncommitted changes (this includes both staged and unstaged tracked files)
    let local_args = vec!["diff", "--name-only", "HEAD"];
    if let Ok(files) = run_git_diff(&local_args) {
        all_files.extend(files);
    }
    
    // 3. Untracked files
    let untracked_args = vec!["ls-files", "--others", "--exclude-standard"];
    if let Ok(files) = run_git_diff(&untracked_args) {
        all_files.extend(files);
    }

    let mut result: Vec<String> = all_files.into_iter().collect();
    result.sort();
    
    Ok(result)
}
