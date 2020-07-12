use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Root {
    // require
    path: String,

    // optional
    #[serde(default)]
    readonly: bool,
}
