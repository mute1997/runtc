mod root;
mod mount;
mod process;
mod linux;
mod hooks;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

use root::Root;
use mount::Mount;
use process::Process;
use linux::Linux;
use hooks::Hooks;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // required
    #[serde(alias = "ociVersion")]
    oci_version: String,

    // optional
    #[serde(default)]
    hooks: Hooks,

    #[serde(default)]
    root: Root,

    #[serde(default)]
    mounts: Vec<Mount>,

    #[serde(default)]
    process: Process,

    #[serde(default)]
    hostname: String,

    #[serde(default)]
    linux: Linux,
}

impl Config {
    pub fn new(bundle: String) -> Result<Config> {
        let file = File::open(format!("{}/config.json", bundle))?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    pub fn create(&self) -> Result<()> {
        self.linux.create()?;
        Ok(())
    }

    pub fn delete(&self) -> Result<()> {
        self.linux.delete()?;
        Ok(())
    }

    pub fn kill(&self) -> Result<()> {
        self.linux.kill()?;
        Ok(())
    }

    pub fn start(&self) -> Result<()> {
        self.linux.start()?;
        Ok(())
    }

    fn run_hook(&self, hook_type: String) -> Result<()> {
        let type_str = hook_type.as_str();
        let hooks = match type_str {
            "prestart" => &self.hooks.prestart,
            "create_runtime" => &self.hooks.create_runtime,
            "create_container" => &self.hooks.create_container,
            "start_container" => &self.hooks.start_container,
            "poststart" => &self.hooks.poststart,
            "poststop" => &self.hooks.poststop,
            _ => return Ok(()),
        };
        for hook in hooks {
            match hook.run() {
                Err(e) => return Err(e),
                _ => {},
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn should_create_valid() {
        let bundle = format!("{}/resources/test/config_valid",
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).to_str().unwrap());
        Config::new(bundle).unwrap();
        // let config =  Config::new(bundle);
        // assert!(config.is_ok())
    }
}
