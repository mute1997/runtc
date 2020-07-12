pub mod resources;
pub mod device;
pub mod mapping;
pub mod namespace;
pub mod seccomp;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::config::linux::{
    namespace::Namespace,
    device::Device,
    seccomp::Seccomp,
    mapping::Mapping,
    resources::Resources,
};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Linux {
    // optional
    #[serde(default)]
    pub namespaces: Vec<Namespace>,

    #[serde(default)]
    pub devices: Vec<Device>,

    #[serde(default)]
    pub seccomp: Seccomp,

    #[serde(default)]
    pub sysctl: HashMap<String, String>,

    #[serde(default)]
    pub resources: Resources,

    #[serde(default, alias = "cgroupsPath")]
    pub cgroups_path: String,

    #[serde(default, alias = "uidMappings")]
    pub uid_mappings: Vec<Mapping>,

    #[serde(default, alias = "gidMappings")]
    pub gid_mappings: Vec<Mapping>,

    #[serde(default, alias = "rootfsPropagation")]
    pub rootfs_propagation: String,

    #[serde(default, alias = "maskedPaths")]
    pub masked_paths: Vec<String>,

    #[serde(default, alias = "readonlyPaths")]
    pub readonly_paths: Vec<String>,

    #[serde(default, alias = "mountLabel")]
    pub mount_label: String,
}

impl Linux {
    pub fn create(&self) -> Result<()> {
        self.resources.create(&self.cgroups_path)?;
        Ok(())
    }

    pub fn delete(&self) -> Result<()> {
        self.resources.delete(&self.cgroups_path)?;
        Ok(())
    }

    pub fn kill(&self) -> Result<()> {
        Ok(())
    }

    pub fn start(&self) -> Result<()> {
        Ok(())
    }
}

