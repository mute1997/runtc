use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, remove_dir_all};

use crate::utils::{find_cgroup_base, write_to_cgroup};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Memory {
    #[serde(default)]
    limit: u64,

    #[serde(default)]
    reservation: u64,

    #[serde(default)]
    swap: u64,

    #[serde(default)]
    kernel: u64,

    #[serde(default, alias = "kernelTCP")]
    kernel_tcp: u64,

    #[serde(default)]
    swappiness: u64,

    #[serde(default, alias = "disableOOMKiller")]
    disable_oom_killer: bool,
}

impl Memory {
    pub fn create(&self, cgroups_path: &String) -> Result<()> {
        let cgroups_base = find_cgroup_base()?;
        let memory_base = format!("{}/memory/{}/", cgroups_base, cgroups_path);

        // create cgroup
        create_dir_all(memory_base.clone()).with_context(|| format!("Failed to create {}.", memory_base))?;

        // set cgroup
        if self.limit != 0 {
            write_to_cgroup(self.limit, format!("{}/memory.limit_in_bytes", memory_base))?;
        }

        if self.reservation != 0 {
            write_to_cgroup(self.reservation, format!("{}/memory.soft_limit_in_bytes", memory_base))?;
        }

        if self.swap != 0 {
            write_to_cgroup(self.swap, format!("{}/memory.memsw.limit_in_bytes", memory_base))?;
        }

        if self.kernel != 0 {
            write_to_cgroup(self.kernel, format!("{}/memory.kmem.limit_in_bytes", memory_base))?;
        }

        if self.kernel_tcp != 0 {
            write_to_cgroup(self.kernel_tcp, format!("{}/memory.kmem.tcp.limit_in_bytes", memory_base))?;
        }

        if self.swappiness != 0 {
            write_to_cgroup(self.swappiness, format!("{}/memory.swappiness", memory_base))?;
        }

        write_to_cgroup(self.disable_oom_killer as i32, format!("{}/memory.oom_control", memory_base))?;

        Ok(())
    }

    pub fn delete(&self, cgroups_path: &String) -> Result<()> {
        let cgroups_base = find_cgroup_base()?;
        remove_dir_all(format!("{}/memory/{}", cgroups_base, cgroups_path))?;
        Ok(())
    }
}
