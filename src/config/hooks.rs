use anyhow::{Result, Error};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Hooks {
    // optional
    #[serde(default)]
    pub prestart: Vec<Hook>,

    #[serde(default, alias = "createRuntime")]
    pub create_runtime: Vec<Hook>,

    #[serde(default, alias = "createContainer")]
    pub create_container: Vec<Hook>,

    #[serde(default, alias = "startContinaer")]
    pub start_container: Vec<Hook>,

    #[serde(default)]
    pub poststart: Vec<Hook>,

    #[serde(default)]
    pub poststop: Vec<Hook>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Hook {
    // require
    path: String,

    // optional
    #[serde(default)]
    args: Vec<String>,

    #[serde(default)]
    env: Vec<String>,

    #[serde(default)]
    timeout: u32,
}

impl Hook {
    pub fn run(&self) -> Result<()> {
        let mut cmd = Command::new(&self.path);
        for arg in &self.args {
            cmd.arg(arg);
        }
        match cmd.output() {
            Err(e) => Err(Error::new(e)),
            _ => Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{path::Path, fs::remove_file};

    #[test]
    fn should_run_hook() {
        let _ = remove_file("/tmp/hook");
        let hook = Hook {
            path: "/usr/bin/touch".to_string(),
            args: vec!("/tmp/hook".to_string()),
            env: vec!(),
            timeout: 0,
        };
        let _ = hook.run();
        assert!(Path::new("/tmp/hook").exists());
    }

    #[test]
    fn should_run_hook_with_multi_args() {
        let _ = remove_file("/tmp/hook");
        let _ = remove_file("/tmp/hook2");
        let hook = Hook {
            path: "/usr/bin/touch".to_string(),
            args: vec!(
                "/tmp/hook".to_string(),
                "/tmp/hook2".to_string(),
            ),
            env: vec!(),
            timeout: 0,
        };
        let _ = hook.run();
        assert!(Path::new("/tmp/hook").exists());
        assert!(Path::new("/tmp/hook2").exists());
    }

    // TODO env
    // TODO timeout
}
