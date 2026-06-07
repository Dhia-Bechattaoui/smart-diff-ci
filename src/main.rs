use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::Path;

mod git;
mod graph;
mod parser;
mod crawler;
mod config;

use crawler::DependencyCrawler;
use config::Config;

#[derive(Parser)]
#[command(name = "smart-diff")]
#[command(about = "A language-agnostic CLI tool for Test Impact Analysis", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze the workspace and list affected test files without running them
    Analyze {
        /// Base branch or commit to compare against (e.g. origin/main)
        #[arg(short, long, default_value = "origin/main")]
        base: String,
    },
    /// Find affected tests and execute them automatically
    Run {
        /// Base branch or commit to compare against (e.g. origin/main)
        #[arg(short, long, default_value = "origin/main")]
        base: String,
        
        /// The test runner command to execute (e.g., -- flutter test)
        #[arg(last = true)]
        runner_args: Vec<String>,
    },
}

/// A dynamic heuristic to identify test files.
fn is_test_file(path: &str, config: &Config) -> bool {
    for regex in &config.compiled_patterns {
        if regex.is_match(path) {
            return true;
        }
    }
    false
}

fn resolve_impacted_tests(base: &str, config: &Config) -> Result<Vec<String>> {
    println!("🚀 Building dependency graph for the workspace...");
    let crawler = DependencyCrawler::new();
    let graph = crawler.build_graph(Path::new("."));

    println!("🔍 Analyzing workspace against base: {}", base);
    let changed_files = git::get_changed_files(base)?;
    println!("📝 Found {} changed files. Resolving impact...", changed_files.len());
    
    let affected = graph.find_affected_files(&changed_files);
    let mut affected_tests: Vec<String> = affected
        .into_iter()
        .filter(|f| is_test_file(&f, config))
        .collect();
        
    affected_tests.sort();
    Ok(affected_tests)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load();

    match &cli.command {
        Commands::Analyze { base } => {
            match resolve_impacted_tests(base, &config) {
                Ok(affected_tests) => {
                    println!("\n🎯 Affected Test Files ({}):", affected_tests.len());
                    for test in &affected_tests {
                        println!(" - {}", test);
                    }

                    if affected_tests.is_empty() {
                        println!("✅ No tests need to be run!");
                    }
                }
                Err(e) => println!("❌ Error resolving impacted tests: {}", e),
            }
        }
        Commands::Run { base, runner_args } => {
            if runner_args.is_empty() {
                anyhow::bail!("You must provide a test runner command.\nExample: smart-diff run --base origin/main -- flutter test");
            }

            match resolve_impacted_tests(base, &config) {
                Ok(affected_tests) => {
                    if affected_tests.is_empty() {
                        println!("✅ No tests need to be run!");
                        return Ok(());
                    }

                    println!("\n🚀 Executing test runner on {} affected files...", affected_tests.len());
                    
                    let program = &runner_args[0];
                    let mut args = runner_args[1..].to_vec();
                    args.extend(affected_tests);

                    let status = std::process::Command::new(program)
                        .args(&args)
                        .status()
                        .map_err(|e| anyhow::anyhow!("Failed to spawn test runner '{}': {}", program, e))?;

                    if !status.success() {
                        std::process::exit(status.code().unwrap_or(1));
                    }
                }
                Err(e) => {
                    println!("❌ Error resolving impacted tests: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
