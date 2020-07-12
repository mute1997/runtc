use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, remove_dir_all};

use crate::utils::{find_cgroup_base, write_to_cgroup};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Device {
    #[serde(default)]
    path: String,

    #[serde(default, alias = "type")]
    device_type: String,

    #[serde(default)]
    major: u64,

    #[serde(default)]
    minor: u64,

    #[serde(default, alias = "fileMode")]
    file_mode: u64,

    #[serde(default)]
    uid: u64,

    #[serde(default)]
    gid: u64,
}

impl Device {
    pub fn create(&self, cgroups_path: &String) -> Result<()> {
        let cgroups_base = find_cgroup_base()?;
        let devices_base = format!("{}/devices/{}/", cgroups_base, cgroups_path);

        // create cgroup
        create_dir_all(devices_base.clone())?;

        // TODO not ok devices c 10:229 rwm is set correctly
        // set cgroup
        if self.major != 0 {
            let content = format!("{} {}:{} rwm\n", self.device_type, self.major, self.minor);
            let path = format!("{}/devices.allow", devices_base);
            write_to_cgroup(content, path)?
        }

        Ok(())
    }

    pub fn delete(&self, cgroups_path: &String) -> Result<()> {
        let cgroups_base = find_cgroup_base()?;
        remove_dir_all(format!("{}/devices/{}", cgroups_base, cgroups_path))?;
        Ok(())
    }
}
