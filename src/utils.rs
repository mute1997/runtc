use anyhow::{Context, Result};
use std::fs::{File, create_dir_all};
use std::io::prelude::*;

// TODO fix
pub fn find_cgroup_base() -> Result<String> {
    Ok("/sys/fs/cgroup".to_string())
}

pub fn find_container_path() -> Result<String> {
    let path = "/tmp/runtc".to_string();
    create_dir_all(path.clone()).with_context(|| format!("Failed to create dir {}.", path))?;
    Ok(path)
}

pub fn write_to_cgroup<T: std::fmt::Display>(content: T, path: String) -> Result<()> {
    let mut file = File::create(path.clone())
        .with_context(|| format!("Failed to open. path: {}", path))?;
    file.write_all(content.to_string().as_bytes())
        .with_context(|| format!("Failed to write. path: {}, content: {}", path, content.to_string()))?;
    Ok(())
}
