use clap::Parser;
use std::env::var_os;
use std::ffi::OsString;
use std::io::Result;
use std::process::{exit, Command};
use tempdir::TempDir;

/// Execute a shell within a temporary directory, removing it after the shell exits
#[derive(Parser)]
#[clap()]
struct Options {}

fn main() -> Result<()> {
    Options::parse();
    let tmpdir = TempDir::new(env!("CARGO_PKG_NAME"))?;
    let shell = var_os("SHELL").unwrap_or_else(|| OsString::from("/bin/sh"));
    println!("Executing: {:?} in {:?}", &shell, tmpdir.as_ref().display());
    let status = Command::new(shell).current_dir(&tmpdir).status()?;
    let code = status.code();
    println!(
        "Exit code {:?}; removing {:?}",
        code,
        tmpdir.as_ref().display()
    );
    exit(code.unwrap_or(-1))
}
