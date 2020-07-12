use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Mapping {
    // required
    #[serde(alias = "containerID")]
    container_id: u32,

    #[serde(alias = "hostID")]
    host_id: u32,

    size: u32,
}
