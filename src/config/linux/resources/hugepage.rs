use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::utils::{find_cgroup_base, write_to_cgroup};
use std::fs::{remove_dir_all, create_dir_all};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Hugepage {
    #[serde(default, alias = "pageSize")]
    page_size: String,

    #[serde(default)]
    limit: u64,
}

impl Hugepage {
    pub fn create(&self, cgroups_path: &String) -> Result<()> {
        let cgroups_base = find_cgroup_base()?;
        let hugetlb_base = format!("{}/hugetlb/{}/", cgroups_base, cgroups_path);

        // create cgroup
        create_dir_all(hugetlb_base.clone())?;

        // set cgroup
        write_to_cgroup(self.limit, format!("{}/hugetlb.{}.limit_in_bytes", hugetlb_base, self.page_size))?;

        Ok(())
    }

    pub fn delete(&self, cgroups_path: &String) -> Result<()> {
        let cgroups_base = find_cgroup_base()?;
        remove_dir_all(format!("{}/hugetlb/{}", cgroups_base, cgroups_path))?;
        Ok(())
    }
}
