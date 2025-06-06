use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Enable debug output
    #[arg(short, long)]
    pub debug: bool,

    /// Path to the configuration file (YAML or JSON)
    #[arg(short = 'f', long)]
    pub config: Option<PathBuf>,

    /// Template category
    #[arg(short = 'c', long)]
    pub category: Option<String>,

    /// Template name
    #[arg(short = 'n', long)]
    pub template: Option<String>,

    /// Trigger GitHub workflow instead of local generation
    #[arg(long)]
    pub remote: bool,

    /// GitHub token for remote workflow
    #[arg(long)]
    pub token: Option<String>,
}
