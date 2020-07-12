use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Mount {
    // require
    destination: String,

    // optional
    #[serde(default)]
    source: String,

    #[serde(default)]
    options: Vec<String>,
}
