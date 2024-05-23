mod cli;
mod package_list;
mod install;
mod utils;

use cli::{Cli, Commands};
use clap::Parser;
use package_list::PackageList;
use std::path::Path;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Track { package } => {
            let mut pkg_list = PackageList::load_or_new();
            pkg_list.add(package.clone());
            pkg_list.save().expect("Failed to save package list");
            println!("Tracked package: {}", package);
        }
        Commands::GenerateFile { file } => {
            let pkg_list = PackageList::load_or_new();
            pkg_list.save_to_file(Path::new(file)).expect("Failed to save package list to file");
            println!("Package list saved to: {}", file);
        }
        Commands::InstallFromFile { file } => {
            let pkg_list = PackageList::load_from_file(Path::new(file)).expect("Failed to load package list from file");
            for package in pkg_list.packages {
                install::install_package(&package).expect("Failed to install package");
            }
        }
    }
}
