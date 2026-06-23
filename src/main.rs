mod case;
mod collectors;
mod db;
mod hashing;
mod models;
mod report;
mod rules;
mod scanner;
mod util;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

const APP: &str = "COLDCASE";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "xtr4ng3";

#[derive(Parser, Debug)]
#[command(name = "coldcase")]
#[command(author = "xtr4ng3")]
#[command(version)]
#[command(about = "Local forensic triage workbench", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new local case workspace.
    New {
        /// Case directory to create.
        case_dir: PathBuf,

        /// Optional case title.
        #[arg(short, long)]
        title: Option<String>,
    },

    /// Capture a system snapshot into a case.
    Snapshot {
        /// Case directory.
        case_dir: PathBuf,
    },

    /// Scan a folder and store file evidence.
    Scan {
        /// Case directory.
        case_dir: PathBuf,

        /// Folder to scan.
        target: PathBuf,

        /// Maximum file size in MB to hash fully.
        #[arg(long, default_value_t = 256)]
        max_mb: u64,
    },

    /// Build a timeline from collected evidence.
    Timeline {
        /// Case directory.
        case_dir: PathBuf,
    },

    /// Generate HTML, JSON and SARIF reports.
    Report {
        /// Case directory.
        case_dir: PathBuf,
    },

    /// Print case status.
    Status {
        /// Case directory.
        case_dir: PathBuf,
    },

    /// Create a default rules file in the current folder.
    Rules {
        /// Output path for rules file.
        #[arg(default_value = "coldcase.rules")]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { case_dir, title } => case::create_case(&case_dir, title)?,
        Commands::Snapshot { case_dir } => collectors::collect_snapshot(&case_dir)?,
        Commands::Scan { case_dir, target, max_mb } => scanner::scan_target(&case_dir, &target, max_mb)?,
        Commands::Timeline { case_dir } => report::generate_timeline(&case_dir)?,
        Commands::Report { case_dir } => report::generate_reports(&case_dir)?,
        Commands::Status { case_dir } => case::status(&case_dir)?,
        Commands::Rules { output } => rules::write_default_rules(&output)?,
    }

    Ok(())
}
