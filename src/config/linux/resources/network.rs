use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Network {}

impl Network {
    pub fn create(&self, cgroups_path: &String) -> Result<()> {
        Ok(())
    }

    pub fn delete(&self, cgroups_path: &String) -> Result<()> {
        Ok(())
    }
}
