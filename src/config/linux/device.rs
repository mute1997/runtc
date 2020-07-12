use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Device {
    // required
    path: String,
    major: i64,
    minor: i64,

    #[serde(alias = "type")]
    device_type: String,

    // optional
    #[serde(default)]
    uid: u32,

    #[serde(default)]
    gid: u32,

    #[serde(default, alias = "fileMode")]
    file_mode: u32,
}
