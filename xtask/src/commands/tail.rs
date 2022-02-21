use anyhow::Result;
use structopt::StructOpt;

use crate::{tools::Runner, PKG_PROJECT_ROOT};

#[derive(Debug, StructOpt)]
pub struct Tail {}

impl Tail {
    pub fn run(&self, verbose: bool) -> Result<()> {
        let runner = Runner::new("tail", verbose)?;
        let log_file = "./tui-snek.log";
        crate::info!("recreating {}", log_file);
        let _ = std::fs::remove_file(log_file);
        std::fs::File::create(log_file)?;
        crate::info!("you should run `cargo run` in another terminal to see logs",);
        runner.exec(&["-f", log_file], &PKG_PROJECT_ROOT, None)?;
        Ok(())
    }
}
