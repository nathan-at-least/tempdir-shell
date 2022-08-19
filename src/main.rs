use clap::Parser;
use std::env::var_os;
use std::ffi::OsString;
use std::io::Result;
use std::process::{exit, Command};
use tempdir::TempDir;

/// Execute a shell within a temporary directory, removing it after the shell exits
#[derive(Parser)]
#[clap()]
struct Options {
    /// The shell or executable to execute in the temporary directory
    #[clap(long, short, default_value_t = var_os("SHELL").unwrap_or_else(|| OsString::from("/bin/sh")).to_str().unwrap().to_string())]
    shell: String,

    /// Arguments to the shell or executable
    #[clap()]
    args: Vec<String>,
}

fn main() -> Result<()> {
    const NAME: &str = env!("CARGO_PKG_NAME");

    let opts = Options::parse();

    let mut cmd = Command::new(opts.shell);
    let tmpdir = TempDir::new(env!("CARGO_PKG_NAME"))?;
    println!(
        "{}: created directory {:?}",
        NAME,
        tmpdir.as_ref().display()
    );
    cmd.current_dir(&tmpdir);
    cmd.args(&opts.args);
    println!("{}: executing {:?}", NAME, &cmd);

    let status = cmd.status()?;

    let code = status.code();

    if let Some(n) = code {
        println!("{}: exit code {}", NAME, n,);
    } else {
        println!("{}: exit code <n/a>", NAME,);
    }

    println!("{}: removing {:?}", NAME, tmpdir.as_ref().display());
    exit(code.unwrap_or(-1))
}
