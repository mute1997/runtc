use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Namespace {
    // required
    #[serde(alias = "type")]
    namespace_type: String,

    // optional
    #[serde(default)]
    path: String
}
