use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::utils::{find_cgroup_base, write_to_cgroup};
use std::fs::{create_dir_all, remove_dir_all};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Cpu {
    #[serde(default)]
    shares: u64,

    #[serde(default)]
    quota: u64,

    #[serde(default)]
    period: u64,

    #[serde(default, alias = "realtimeRuntime")]
    realtime_runtime: u64,

    #[serde(default, alias = "realtimePeriod")]
    realtime_period: u64,

    #[serde(default)]
    cpus: String,

    #[serde(default)]
    mems: String,
}

impl Cpu {
    pub fn create(&self, cgroups_path: &String) -> Result<()> {
        log::info!("cpu self {:?}", self);

        let cgroups_base = find_cgroup_base()?;
        let cpu_base = format!("{}/cpu/{}/", cgroups_base, cgroups_path);
        let cpuset_base = format!("{}/cpuset/{}/", cgroups_base, cgroups_path);

        // create cgroup
        create_dir_all(cpu_base.clone()).with_context(|| format!("Failed to create {}.", cpu_base))?;
        create_dir_all(cpuset_base.clone()).with_context(|| format!("Failed to create {}.", cpuset_base))?;

        // set cgroup
        if self.shares != 0 {
            write_to_cgroup(self.shares, format!("{}/cpu.shares", cpu_base))?;
        }

        if self.quota != 0 {
            write_to_cgroup(self.quota, format!("{}/cpu.cfs_quota_us", cpu_base))?;
        }

        if self.period != 0 {
            write_to_cgroup(self.period, format!("{}/cpu.cfs_period_us", cpu_base))?;
        }

        if self.cpus != "" {
            write_to_cgroup(&self.cpus, format!("{}/cpuset.cpus", cpuset_base))?;
        }

        if self.mems != "" {
            write_to_cgroup(&self.mems, format!("{}/cpuset.mems", cpuset_base))?;
        }

        Ok(())
    }

    pub fn delete(&self, cgroups_path: &String) -> Result<()> {
        let cgroups_base = find_cgroup_base()?;
        remove_dir_all(format!("{}/cpu/{}", cgroups_base, cgroups_path))?;
        remove_dir_all(format!("{}/cpuset/{}", cgroups_base, cgroups_path))?;
        Ok(())
    }
}
