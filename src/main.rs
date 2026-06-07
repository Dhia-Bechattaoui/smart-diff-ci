mod git;
mod graph;
mod parser;

use anyhow::Result;
use clap::{Parser, Subcommand};

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Analyze { base } => {
            println!("Analyzing workspace against base: {}", base);
            match git::get_changed_files(base) {
                Ok(files) => {
                    println!("Found {} changed files:", files.len());
                    for file in files {
                        println!(" - {}", file);
                    }
                }
                Err(e) => println!("Error getting changed files: {}", e),
            }
            // TODO: Implement dependency graph analysis and test resolution
        }
        Commands::Run { base } => {
            println!("Running affected tests against base: {}", base);
            // TODO: Implement analysis and test execution
        }
    }

    Ok(())
}
