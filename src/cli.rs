use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pkg_tracker")]
#[command(about = "A tool to track and reinstall packages")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Track {
        #[arg(short, long)]
        package: String,
    },
    GenerateFile {
        #[arg(short, long)]
        file: String,
    },
    InstallFromFile {
        #[arg(short, long)]
        file: String,
    },
}
