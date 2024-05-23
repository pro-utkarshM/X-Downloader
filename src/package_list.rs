use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct PackageList {
    pub packages: Vec<String>,
}

impl PackageList {
    pub fn new() -> Self {
        PackageList { packages: Vec::new() }
    }

    pub fn load_or_new() -> Self {
        match Self::load() {
            Ok(pkg_list) => pkg_list,
            Err(_) => Self::new(),
        }
    }

    pub fn add(&mut self, package: String) {
        if !self.packages.contains(&package) {
            self.packages.push(package);
        }
    }

    pub fn save(&self) -> io::Result<()> {
        self.save_to_file(Path::new("packages.json"))
    }

    pub fn save_to_file(&self, path: &Path) -> io::Result<()> {
        let file = File::create(path)?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }

    pub fn load() -> io::Result<Self> {
        Self::load_from_file(Path::new("packages.json"))
    }

    pub fn load_from_file(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;
        let package_list = serde_json::from_reader(file)?;
        Ok(package_list)
    }
}
