pub mod cpu;
pub mod blockio;
pub mod device;
pub mod hugepage;
pub mod memory;
pub mod network;
pub mod pids;
pub mod intelrdt;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::config::linux::resources::{
    network::Network,
    cpu::Cpu,
    memory::Memory,
    blockio::BlockIO,
    pids::Pids,
    hugepage::Hugepage,
    device::Device as LinuxDevice,
    intelrdt::IntelRdt,
};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Resources {
    // optional
    #[serde(default)]
    network: Network,

    #[serde(default)]
    memory: Memory,

    #[serde(default)]
    cpu: Cpu,

    #[serde(default)]
    devices: Vec<LinuxDevice>,

    #[serde(default, alias = "blockIO")]
    block_io: BlockIO,

    #[serde(default, alias = "hugepageLimits")]
    hugepage: Vec<Hugepage>,

    #[serde(default)]
    pids: Pids,

    #[serde(default, alias = "intelRdt")]
    intel_rdt: IntelRdt,
}


impl Resources {
    // TODO fix
    pub fn create(&self, cgroups_path: &String) -> Result<()> {
        self.network.create(cgroups_path)?;
        self.memory.create(cgroups_path)?;
        self.cpu.create(cgroups_path)?;
        self.block_io.create(cgroups_path)?;
        self.pids.create(cgroups_path)?;
        self.intel_rdt.create(cgroups_path)?;

        for device in &self.devices {
            device.create(cgroups_path)?;
        }

        for hugepage in &self.hugepage {
            hugepage.create(cgroups_path)?;
        }

        Ok(())
    }

    pub fn delete(&self, cgroups_path: &String) -> Result<()> {
        self.network.delete(cgroups_path)?;
        self.memory.delete(cgroups_path)?;
        self.cpu.delete(cgroups_path)?;
        self.block_io.delete(cgroups_path)?;
        self.pids.delete(cgroups_path)?;
        self.intel_rdt.delete(cgroups_path)?;

        for device in &self.devices {
            device.delete(cgroups_path)?;
        }

        for hugepage in &self.hugepage {
            hugepage.delete(cgroups_path)?;
        }

        Ok(())
    }
}
