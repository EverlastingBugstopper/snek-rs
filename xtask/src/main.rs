mod commands;
mod tools;

pub(crate) use tools::Runner;

use ansi_term::Colour::Green;
use anyhow::{anyhow, Context, Result};
use camino::Utf8PathBuf;
use lazy_static::lazy_static;
use structopt::StructOpt;

lazy_static! {
    pub(crate) static ref PKG_PROJECT_ROOT: Utf8PathBuf =
        project_root().expect("Could not find the project root.");
}

fn project_root() -> Result<Utf8PathBuf> {
    let manifest_dir = Utf8PathBuf::try_from(env!("CARGO_MANIFEST_DIR"))
        .with_context(|| "Could not find the root directory.")?;
    let root_dir = manifest_dir
        .ancestors()
        .nth(1)
        .ok_or_else(|| anyhow!("Could not find project root."))?;
    Ok(root_dir.to_path_buf())
}

fn main() -> Result<()> {
    let app = Xtask::from_args();
    app.run()
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "xtask",
    about = "Workflows used locally and in CI for developing snek-rs"
)]
struct Xtask {
    #[structopt(subcommand)]
    pub command: Command,

    /// Specify xtask's verbosity level
    #[structopt(long = "verbose", short = "v", global = true)]
    verbose: bool,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Lint snek_rs
    Lint(commands::Lint),

    /// Tail logs for snek_rs
    Tail(commands::Tail),

    /// Run all available tests for snek_rs
    Test(commands::Test),
}

impl Xtask {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Command::Lint(command) => command.run(self.verbose),
            Command::Tail(command) => command.run(self.verbose),
            Command::Test(command) => command.run(self.verbose),
        }?;
        eprintln!("{}", Green.bold().paint("Success!"));
        Ok(())
    }
}
