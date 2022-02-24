use camino::Utf8PathBuf;

use crate::{Result, Runner, PKG_PROJECT_ROOT};

use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;

pub(crate) struct CargoRunner {
    cargo_package_directory: Utf8PathBuf,
    runner: Runner,
    env: HashMap<String, String>,
}

impl CargoRunner {
    pub(crate) fn new(verbose: bool) -> Result<Self> {
        Self::new_with_path(verbose, PKG_PROJECT_ROOT.clone())
    }

    pub(crate) fn new_with_path<P>(verbose: bool, cargo_package_directory: P) -> Result<Self>
    where
        P: Into<Utf8PathBuf>,
    {
        let runner = Runner::new("cargo", verbose)?;
        Ok(CargoRunner {
            cargo_package_directory: cargo_package_directory.into(),
            runner,
            env: HashMap::new(),
        })
    }

    pub(crate) fn lint(&self) -> Result<()> {
        self.cargo_exec(vec!["fmt", "--all"], vec!["--check"])?;
        self.cargo_exec(vec!["clippy", "--all"], vec!["-D", "warnings"])?;
        Ok(())
    }

    pub(crate) fn test(&self) -> Result<()> {
        self.cargo_exec(vec!["nextest", "run", "--workspace", "--locked"], vec![])?;
        Ok(())
    }

    pub(crate) fn cargo_exec(&self, cargo_args: Vec<&str>, extra_args: Vec<&str>) -> Result<()> {
        let mut args = cargo_args;
        if !extra_args.is_empty() {
            args.push("--");
            for extra_arg in extra_args {
                args.push(extra_arg);
            }
        }
        let env = if self.env.is_empty() {
            None
        } else {
            Some(&self.env)
        };
        self.runner.exec(&args, &self.cargo_package_directory, env)
    }
}

fn _copy_dir_all(source: &Utf8PathBuf, destination: &Utf8PathBuf) -> Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(&source)?.flatten() {
        if let Ok(file_type) = entry.file_type() {
            if let Some(file_name) = entry.file_name().to_str() {
                let this_destination = destination.join(file_name);
                let this_source = entry.path().try_into()?;
                if file_type.is_dir() {
                    _copy_dir_all(&this_source, &this_destination)?;
                } else {
                    fs::copy(this_source, this_destination)?;
                }
            }
        }
    }
    Ok(())
}
