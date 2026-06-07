use anyhow::{Context, Result};
use std::process::Command;

/// Retrieves a list of changed files between the given base branch and HEAD.
pub fn get_changed_files(base_branch: &str) -> Result<Vec<String>> {
    let output = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg(format!("{}...HEAD", base_branch))
        .output()
        .context("Failed to execute git diff command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git diff failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let files: Vec<String> = stdout
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(files)
}
