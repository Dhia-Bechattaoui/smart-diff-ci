use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::Path;

mod git;
mod graph;
mod parser;
mod crawler;

use crawler::DependencyCrawler;

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
    },
}

/// A simple heuristic to identify test files for the MVP.
fn is_test_file(path: &str) -> bool {
    path.ends_with("_test.dart")
        || path.ends_with(".test.ts")
        || path.ends_with(".test.js")
        || path.ends_with(".spec.ts")
        || path.ends_with(".spec.js")
        || (path.starts_with("test_") && path.ends_with(".py"))
        || path.ends_with("_test.py")
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Analyze { base } => {
            println!("🚀 Building dependency graph for the workspace...");
            let crawler = DependencyCrawler::new();
            let graph = crawler.build_graph(Path::new("."));

            println!("🔍 Analyzing workspace against base: {}", base);
            match git::get_changed_files(base) {
                Ok(changed_files) => {
                    println!("📝 Found {} changed files. Resolving impact...", changed_files.len());
                    
                    let affected = graph.find_affected_files(&changed_files);
                    let mut affected_tests: Vec<String> = affected
                        .into_iter()
                        .filter(|f| is_test_file(f))
                        .collect();
                        
                    affected_tests.sort();
                    
                    println!("\n🎯 Affected Test Files ({}):", affected_tests.len());
                    for test in &affected_tests {
                        println!(" - {}", test);
                    }

                    if affected_tests.is_empty() {
                        println!("✅ No tests need to be run!");
                    }
                }
                Err(e) => println!("❌ Error getting changed files: {}", e),
            }
        }
        Commands::Run { base: _ } => {
            // TODO: Implement the Runner module
            println!("🚀 The 'run' command will be implemented in the next phase!");
        }
    }

    Ok(())
}
