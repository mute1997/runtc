pub mod state;

use anyhow::{Result, Context};
use std::fs::{remove_file, File};
use std::io::{BufReader, prelude::*};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::container::state::State;
use crate::utils::find_container_path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    id: String,
    bundle: String,
    path: String,
    state: State,
    config: Config,
}

impl Container {
    pub fn new(id: String, bundle: String) -> Result<Container> {
        let path = format!("{}/{}.json", find_container_path()?, id.clone());
        let config = Config::new(bundle.clone())?;
        let state = State::new(id.clone(), bundle.clone());
        Ok(Container {
            id,
            bundle,
            path,
            state,
            config,
        })
    }

    pub fn find(id: String) -> Result<Container> {
        let path = format!("{}/{}.json", find_container_path()?, id.clone());
        let file = File::open(path.clone())
            .with_context(|| format!("Failed to open. path: {}", path))?;
        let reader = BufReader::new(file);
        let container: Container = serde_json::from_reader(reader)?;
        Ok(container)
    }

    pub fn create(&mut self) -> Result<()> {
        self.config.create()?;
        self.state.create()?;
        self.write()?;
        Ok(())
    }

    pub fn delete(&mut self) -> Result<()> {
        self.config.delete()?;
        self.state.delete()?;
        self.remove()?;
        Ok(())
    }

    pub fn kill(&mut self) -> Result<()> {
        self.config.kill()?;
        self.state.kill()?;
        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        self.config.start()?;
        self.state.start()?;
        Ok(())
    }

    pub fn state(&self) {
        println!("{}", self.state);
    }

    fn write(&self) -> Result<()> {
        let json = serde_json::to_string(&self)?;
        let mut file = File::create(&self.path)?;
        file.write_all(json.as_bytes())?;
        file.sync_all()?;
        Ok(())
    }

    fn remove(&self) -> Result<()> {
        remove_file(&self.path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn should_create() {
        let id = "dummy_id".to_string();
        let bundle = format!("{}/resources/test/config_valid",
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).to_str().unwrap());
        let container = Container::new(id, bundle);
        assert!(container.is_ok())
    }
}
