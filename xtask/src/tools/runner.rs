use anyhow::{anyhow, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use which::which;

use std::collections::HashMap;
use std::convert::TryInto;
use std::process::Command;
use std::str;

pub(crate) struct Runner {
    pub(crate) verbose: bool,
    pub(crate) tool_name: String,
    pub(crate) tool_exe: Utf8PathBuf,
}

impl Runner {
    pub(crate) fn new(tool_name: &str, verbose: bool) -> Result<Self> {
        let tool_exe = which(tool_name).with_context(|| {
            format!(
                "You must have {} installed to run this command.",
                &tool_name
            )
        })?;
        Ok(Runner {
            verbose,
            tool_name: tool_name.to_string(),
            tool_exe: tool_exe.try_into()?,
        })
    }

    pub(crate) fn exec(
        &self,
        args: &[&str],
        directory: &Utf8Path,
        env: Option<&HashMap<String, String>>,
    ) -> Result<()> {
        let full_command = format!("`{} {}`", &self.tool_name, args.join(" "));
        crate::info!("running {} in `{}`", &full_command, directory);
        if self.verbose {
            if let Some(env) = env {
                crate::info!("env:");
                for (key, value) in env {
                    crate::info!("  ${}={}", key, value);
                }
            }
        }

        let mut command = Command::new(&self.tool_exe);
        command.current_dir(directory).args(args);
        if let Some(env) = env {
            command.envs(env);
        }
        let status = command.status()?;
        if status.success() {
            Ok(())
        } else {
            Err(anyhow!("{} failed.", &full_command))
        }
    }
}
