use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Process {
    // require
    cwd: String,

    // optional
    #[serde(default)]
    terminal: bool,

    #[serde(default)]
    user: User,

    #[serde(default)]
    args: Vec<String>,

    #[serde(default)]
    capabilities: Capabilities,

    #[serde(default)]
    rlimits: Vec<Rlimit>,

    #[serde(default, alias = "noNewPrivileges")]
    no_new_privileges: bool,

    #[serde(default, alias = "selinuxLabel")]
    selinux_label: String,

    #[serde(default, alias = "apparmorProfile")]
    apparmor_profile: String,

    #[serde(default, alias = "consoleSize")]
    console_size: ConsoleSize,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct ConsoleSize {
    // require
    height: u32,
    width: u32,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct User {
    // require
    uid: u32,
    gid: u32,

    // optional
    #[serde(default)]
    umask: u32,

    #[serde(default)]
    additional_gids: Vec<u32>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct Capabilities {
    // optional
    #[serde(default)]
    bounding: Vec<String>,

    #[serde(default)]
    permitted: Vec<String>,

    #[serde(default)]
    inheritable: Vec<String>,

    #[serde(default)]
    effective: Vec<String>,

    #[serde(default)]
    ambient: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Rlimit {
    // require
    hard: u32,
    soft: u32,

    #[serde(default, alias = "type")]
    rlimit_type: String,
}
