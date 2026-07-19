
use std::path::Path;
use std::process::{Command, Stdio};

pub fn run(path: &Path, args: Vec<&str>) {
    let status = Command::new(path)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("failed to execute process");

    if !status.success() {
        eprintln!("Process exited with: {:?}", status.code());
    }
}