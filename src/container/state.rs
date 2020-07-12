use std::fmt;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    oci_version: String,
    id: String,
    pub status: String,
    pid: u32,
    bundle: String,
    annotations: HashMap<String, String>,
}

impl State {
    pub fn new(id: String, bundle: String) -> State {
        State{
            id,
            bundle,
            oci_version: "0.2.0".to_string(),
            status: "created".to_string(),
            pid: 0,
            annotations: HashMap::new(),
        }
    }

    pub fn create(&mut self) -> Result<()> {
        self.status = "created".to_string();
        Ok(())
    }

    pub fn delete(&mut self) -> Result<()> {
        if self.status == "stopped" {
            return Err(Error::msg("Container not started."));
        }
        Ok(())
    }

    pub fn kill(&mut self) -> Result<()> {
        if self.status != "created" || self.status != "running" {
            return Err(Error::msg("Container not started."));
        }
        self.status = "stopped".to_string();
        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        self.status = "running".to_string();
        Ok(())
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = match serde_json::to_string(&self) {
            Err(_) => return Ok(()), // TODO error handling
            Ok(c) => c,
        };
        write!(f, "{}", json)
    }
}
