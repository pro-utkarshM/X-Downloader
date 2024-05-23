use std::process::Command;
use std::io;

pub fn install_package(package: &str) -> io::Result<()> {
    Command::new("sudo")
        .arg("apt-get")
        .arg("install")
        .arg("-y")
        .arg(package)
        .spawn()?
        .wait()?;
    Ok(())
}
